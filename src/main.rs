use std::env;
use std::io::Write;
use std::process::Command;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod parser;

#[cfg(test)]
mod parser_test;

const ARGS_SEPARATOR: &str = " ";
const SET_FG_FAILED: &str = "Failed setting a color.";
const FAILED_TO_WRITE_TO_STDOUT: &str = "Failed to write to stdout.";

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
        println!("{}", args);
        return;
    }

    let branch_name = branch_name.unwrap();

    let changes_to_be_commited =
        parser::extract_changes_to_be_commited(&git_status_command_output_string);

    let unstaged_changes = parser::extract_unstaged_changes(&git_status_command_output_string);

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
        .expect(SET_FG_FAILED);
    write!(&mut stdout, "{}", "[").expect(FAILED_TO_WRITE_TO_STDOUT);

    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
        .expect(SET_FG_FAILED);
    write!(&mut stdout, "{}", branch_name).expect(FAILED_TO_WRITE_TO_STDOUT);

    let branch_status = parser::extract_branch_status(&git_status_command_output_string);

    if let Some(bs) = branch_status {
        if bs.ahead > 0 && bs.behind > 0 {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
                .expect(SET_FG_FAILED);

            write!(&mut stdout, " ↑{}↓{}", bs.ahead, bs.behind).expect(FAILED_TO_WRITE_TO_STDOUT);
        } else if bs.ahead > 0 {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                .expect(SET_FG_FAILED);
            write!(&mut stdout, " ↑{}", bs.ahead).expect(FAILED_TO_WRITE_TO_STDOUT);
        } else if bs.behind > 0 {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                .expect(SET_FG_FAILED);
            write!(&mut stdout, " ↓{}", bs.behind).expect(FAILED_TO_WRITE_TO_STDOUT);
        } else if bs.gone {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                .expect(SET_FG_FAILED);
            write!(&mut stdout, " ×",).expect(FAILED_TO_WRITE_TO_STDOUT);
        }
    } else {
        write!(&mut stdout, " ≡").expect(FAILED_TO_WRITE_TO_STDOUT);
    }

    if let Some(ref fc) = changes_to_be_commited {
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
            .expect(SET_FG_FAILED);
        write!(
            &mut stdout,
            " +{} ~{} -{}",
            fc.added, fc.modified, fc.deleted
        )
        .expect(FAILED_TO_WRITE_TO_STDOUT);
    }

    if changes_to_be_commited.is_some() && unstaged_changes.is_some() {
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
            .expect(SET_FG_FAILED);
        write!(&mut stdout, " |").expect(FAILED_TO_WRITE_TO_STDOUT);
    }

    if let Some(ref fc) = unstaged_changes {
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
            .expect(SET_FG_FAILED);
        write!(
            &mut stdout,
            " +{} ~{} -{}",
            fc.added, fc.modified, fc.deleted
        )
        .expect(FAILED_TO_WRITE_TO_STDOUT);
    }

    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
        .expect(SET_FG_FAILED);
    write!(&mut stdout, "{}", "]").expect(FAILED_TO_WRITE_TO_STDOUT);
    stdout
        .set_color(ColorSpec::new().set_reset(true))
        .expect("Failed setting reset.");
}
