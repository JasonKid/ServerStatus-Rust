use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn commit_hash() -> Option<String> {
    Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|hash| hash.trim().into())
}

fn main() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut app_version = String::from(env!("CARGO_PKG_VERSION"));
    if let Some(commit_hash) = commit_hash() {
        app_version = format!("v{} (GIT:{}, BUILD:{})", app_version, commit_hash, now);
    }
    println!("cargo:rustc-env=APP_VERSION={}", app_version);
}
