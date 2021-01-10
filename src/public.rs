use std::env;
const INDEX_HTML: &[u8] = std::include_bytes!(concat!(env!("OUT_DIR"), "/index.html"));
pub fn index_html() -> &'static str {
    std::str::from_utf8(INDEX_HTML).expect("parse index.html as UTF-8 string")
}
