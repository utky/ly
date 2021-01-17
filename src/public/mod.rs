use std::env;
const INDEX_HTML: &[u8] = std::include_bytes!("index.html");
const INDEX_JS: &[u8] = std::include_bytes!(concat!(env!("OUT_DIR"), "/index.js"));
pub fn index_html() -> &'static str {
  std::str::from_utf8(INDEX_HTML).expect("parse index.html as UTF-8 string")
}
pub fn index_js() -> &'static str {
  std::str::from_utf8(INDEX_JS).expect("parse index.js as UTF-8 string")
}

