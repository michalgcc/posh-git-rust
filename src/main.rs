use std::process::Command;

fn main() {

    let mut command_output = Command::new("sh")
        .arg("-c")
        .arg("git status")
        .output()
        .expect("Failed to execute");

    println!("{}", String::from_utf8(command_output.stdout).expect("Aa"));
}
