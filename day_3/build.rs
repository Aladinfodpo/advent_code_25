use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let profile = env::var("PROFILE").unwrap(); // "debug" or "release"

    let target_dir = PathBuf::from("target").join(&profile);

    let src = PathBuf::from("data/input.txt");
    let dst = target_dir.join("input.txt");

    fs::create_dir_all(&target_dir).unwrap();
    fs::copy(&src, &dst).expect("Failed to copy input.txt");

    println!("cargo:rerun-if-changed=data/input.txt");
}
