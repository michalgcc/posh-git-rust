use std::env;
use std::process::Command;

mod parser;

#[cfg(test)]
mod parser_test;

const ARGS_SEPARATOR: &str = " ";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let args = args.join(ARGS_SEPARATOR);

    let git_status_command_output = Command::new("sh")
        .arg("-c")
        .arg("git status --long")
        .output()
        .expect("Failed to execute");

    let git_status_command_output_string =
        String::from_utf8(git_status_command_output.stdout).expect("Expect stdout");

    let branch_name = parser::extract_branch_name(&git_status_command_output_string);

    match branch_name {
        Some(b) => println!("{}[{}]", args, b),
        // Branch name not parsed
        None => println!("{}", args),
    }
}
