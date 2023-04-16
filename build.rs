use std::process::Command;

fn main() {
    Command::new("npm.cmd").arg("run").arg("tailwind").output().expect("failed to execute process");
}