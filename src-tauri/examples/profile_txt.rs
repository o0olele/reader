//! Read-only parser/IPC benchmark: cargo run --example profile_txt -- <TXT path>.
use reader_desktop_lib::infrastructure::ebook::txt;
use std::time::Instant;

fn main() {
    let path = std::env::args().nth(1).expect("TXT path required");
    let start = Instant::now();
    let bytes = std::fs::read(path).expect("read TXT");
    println!(
        "bytes={} read_ms={}",
        bytes.len(),
        start.elapsed().as_millis()
    );
    let start = Instant::now();
    let json = serde_json::to_vec(&bytes).unwrap();
    let encode_ms = start.elapsed().as_millis();
    let start = Instant::now();
    let _: Vec<u8> = serde_json::from_slice(&json).unwrap();
    println!(
        "json_bytes={} encode_ms={encode_ms} decode_ms={}",
        json.len(),
        start.elapsed().as_millis()
    );
    for run in 0..3 {
        let start = Instant::now();
        let parsed = txt::parse(&bytes, "benchmark".into()).unwrap();
        println!(
            "run={run} chapters={} parse_ms={}",
            parsed.chapters.len(),
            start.elapsed().as_millis()
        );
    }
}
