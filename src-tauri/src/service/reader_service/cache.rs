use std::{
    collections::{HashMap, VecDeque},
    sync::{Mutex, OnceLock},
};
static MEMORY_CACHE: OnceLock<Mutex<MemoryChapterCache>> = OnceLock::new();
pub(super) struct MemoryChapterCache {
    entries: HashMap<i64, String>,
    order: VecDeque<i64>,
}
impl MemoryChapterCache {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
            order: VecDeque::new(),
        }
    }
    pub(super) fn get(&mut self, id: i64) -> Option<String> {
        let value = self.entries.get(&id).cloned()?;
        self.order.retain(|item| *item != id);
        self.order.push_back(id);
        Some(value)
    }
    pub(super) fn put(&mut self, id: i64, content: String) {
        self.entries.insert(id, content);
        self.order.retain(|item| *item != id);
        self.order.push_back(id);
        while self.order.len() > 50 {
            if let Some(oldest) = self.order.pop_front() {
                self.entries.remove(&oldest);
            }
        }
    }
    pub(super) fn clear(&mut self) {
        self.entries.clear();
        self.order.clear();
    }
}
pub(super) fn memory_cache() -> &'static Mutex<MemoryChapterCache> {
    MEMORY_CACHE.get_or_init(|| Mutex::new(MemoryChapterCache::new()))
}
