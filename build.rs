use std::process::Command;
use std::env;
use std::path::Path;
fn main() {
  let out_dir = env::var("OUT_DIR").unwrap();
  Command::new("elm")
    .args(&["make", "--output", &format!("{}/index.html", out_dir), "src/Main.elm"]).status().unwrap();
  println!("cargo:rerun-if-changed=src/Main.elm")
}
