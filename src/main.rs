use std::env;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod parser;
mod formatter;

#[cfg(test)]
mod parser_test;

const SET_FG_FAILED: &str = "Failed setting a color.";
const FAILED_TO_WRITE_TO_STDOUT: &str = "Failed to write to stdout.";

fn main() {
    let git_status_command_output_string: String = env::args()
        .nth(1)
        .expect("Expect 'git status --long' output.");

    let git_changes = parser::extract_git_changes(&git_status_command_output_string);

    if git_changes.is_none() {
        return;
    }

    let git_changes = git_changes.unwrap();

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
        .expect(SET_FG_FAILED);
    write!(&mut stdout, "{}", "[").expect(FAILED_TO_WRITE_TO_STDOUT);

    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
        .expect(SET_FG_FAILED);
    write!(&mut stdout, "{}", git_changes.branch_name).expect(FAILED_TO_WRITE_TO_STDOUT);

    if let Some(bs) = git_changes.branch_status {
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
                .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
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

    if let Some(ref fc) = git_changes.staged_changes {
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

    if git_changes.staged_changes.is_some() && git_changes.unstaged_changes.is_some() {
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
            .expect(SET_FG_FAILED);
        write!(&mut stdout, " |").expect(FAILED_TO_WRITE_TO_STDOUT);
    }

    if let Some(ref fc) = git_changes.unstaged_changes {
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
            .expect(SET_FG_FAILED);
        write!(
            &mut stdout,
            " +{} ~{} -{} !",
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
