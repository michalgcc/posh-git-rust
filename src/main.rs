use std::env;
use termcolor::*;

mod formatter;
mod parser;

#[cfg(test)]
mod formatter_test;
#[cfg(test)]
mod parser_test;

fn main() {
    let arg: String = env::args()
        .nth(1)
        .expect("Expect 'git status --long' output.");

    match arg.as_str() {
        "-v" | "--version" => {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            return;
        }
        "-h" | "--help" => {
            println!("GitHub: {}", env!("CARGO_PKG_REPOSITORY"));
            return;
        }
        _ => {}
    }

    let git_changes = parser::extract_git_changes(&arg);

    if git_changes.is_none() {
        return;
    }

    let git_changes = git_changes.unwrap();

    let formatted_prompt_buffer =
        formatter::format_git_status_prompt_buffer(&git_changes, ColorChoice::Auto).unwrap();

    let buffer_writer = BufferWriter::stdout(ColorChoice::Auto);

    buffer_writer.print(&formatted_prompt_buffer).unwrap();
}
