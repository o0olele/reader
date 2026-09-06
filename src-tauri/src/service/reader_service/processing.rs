use super::*;
use crate::{
    repository::replace_rule::SqliteReplaceRuleRepository,
    service::content_processor::ContentProcessor,
    source_engine::{
        legado_rules::LegadoRules,
        rule::{evaluate, expand_template, Extraction, RuleContext},
    },
};

impl ReaderService {
    /// Every read starts from the original chapter text, including cache hits.
    pub async fn read_chapter(&self, chapter_id: i64) -> Result<Chapter, AppError> {
        let mut chapter = self
            .chapters
            .get(chapter_id)
            .await?
            .ok_or_else(|| AppError::InvalidArgument("章节不存在".into()))?;
        let book = self
            .books
            .get(chapter.book_id)
            .await?
            .ok_or_else(|| AppError::InvalidArgument("书籍不存在".into()))?;
        let source = match book.source_id {
            Some(id) => self.sources.get(id).await?,
            None => None,
        };
        if chapter.content.is_empty() {
            if let (Some(source), Some(url)) = (&source, &chapter.remote_url) {
                chapter.content = self
                    .fetch_online_content(source.id, url, Some(chapter_id))
                    .await?;
            }
        }
        let rules = SqliteReplaceRuleRepository::new(self.pool.clone())
            .list()
            .await?;
        // Regex and JS execution must not block an async runtime worker.
        tokio::task::spawn_blocking(move || {
            if let Some(source) = &source {
                if let Some(replace) = LegadoRules::decode(&source.raw_rules)
                    .content
                    .and_then(|rules| rules.replace_regex)
                    .filter(|rule| !rule.trim().is_empty())
                {
                    let mut context = RuleContext::new([
                        ("book.name".into(), book.title.clone()),
                        (
                            "book.author".into(),
                            book.author.clone().unwrap_or_default(),
                        ),
                        ("chapter.title".into(), chapter.title.clone()),
                        ("chapter.index".into(), chapter.number.to_string()),
                    ]);
                    let mut http = source.http_context();
                    http.base_url = chapter.remote_url.clone().unwrap_or(http.base_url);
                    context.with_http(http);
                    let replace = expand_template(&replace, &context);
                    let content = chapter
                        .content
                        .lines()
                        .map(str::trim)
                        .collect::<Vec<_>>()
                        .join("\n");
                    chapter.content =
                        evaluate(&replace, &content, Extraction::Values, &mut context)
                            .map_err(|error| AppError::parse(format!("书源正文替换失败：{error}")))?
                            .join("\n");
                }
            }
            let origin = source
                .as_ref()
                .map(|source| source.base_url.as_str())
                .unwrap_or("");
            let processed = ContentProcessor::new(rules, &book.title, origin)?
                .process(&chapter.title, &chapter.content)?;
            chapter.title = processed.title;
            chapter.content = processed.content;
            Ok(chapter)
        })
        .await
        .map_err(|error| AppError::parse(format!("正文处理失败：{error}")))?
    }
}
