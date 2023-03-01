extern crate open;

use std::process::Command;
use std::str;

fn main() {
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("remote.origin.url")
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(output.stdout.as_slice());
    let split_vector: Vec<&str> = output_str.split("@").collect();

    let mut repo_url: String = "https://".to_owned();
    let cleaned: String = str::replace(split_vector[1], ":", "/").to_owned();
    repo_url.push_str(&cleaned);

    open::that(repo_url).unwrap();
}
