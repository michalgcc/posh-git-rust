use std::env;
use std::io::Write;
use std::process::Command;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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

    let changes_to_be_commited =
        parser::extract_changes_to_be_commited(&git_status_command_output_string);

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
    write!(&mut stdout, "{}", "[");

    // TODO Add branch status
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
    write!(&mut stdout, "{} =", branch_name);

    if let Some(fc) = changes_to_be_commited {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
        write!(
            &mut stdout,
            " +{} ~{} -{}",
            fc.added, fc.modified, fc.deleted
        );
    }

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
    write!(&mut stdout, "{}", "]");
    stdout.set_color(ColorSpec::new().set_reset(true));
}
