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

    if branch_name == None {
        println!("{}", args)
    }

    let branch_name = branch_name.unwrap();

    let mut result = format!("[{}]", branch_name);

    // TODO Add branch status

    let changes_to_be_commited =
        parser::extract_changes_to_be_commited(&git_status_command_output_string);

    if let Some(fc) = changes_to_be_commited {
        result.push_str(&format!(" +{} ~{} -{}", fc.added, fc.modified, fc.deleted))
    }

    println!("{}", result);
}
