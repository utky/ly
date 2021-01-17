use std::env;
use std::process::Command;
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    Command::new("elm")
        .args(&[
            "make",
            "--output",
            &format!("{}/index.js", out_dir),
            "src/Main.elm",
        ])
        .status()
        .unwrap();
    println!("cargo:rerun-if-changed=src/Main.elm")
}
