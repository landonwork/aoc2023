use std::{env, process::Command};

fn main() {
    Command::new("cmd")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .arg("/c")
        .arg(r#"C:\Program Files\nodejs\npx.cmd"#)
        .arg("tailwindcss")
        .arg("-i")
        .arg("./assets/tailwind.css")
        .arg("-o")
        .arg("./static/css/tailwind.css")
        .arg("--watch")
        .output()
        .unwrap();
}