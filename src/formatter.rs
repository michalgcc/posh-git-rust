use crate::parser::GitChanges;
use std::io::Write;
use termcolor::{Buffer, BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

pub fn format_git_status_prompt_buffer(
    git_changes: &GitChanges,
    color_choice: ColorChoice,
) -> Result<Buffer, std::io::Error> {
    let mut buffer = BufferWriter::stdout(color_choice).buffer();

    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
    write!(&mut buffer, "[")?;

    let buffer = append_branch_name(buffer, &git_changes)?;
    let buffer = append_branch_status(buffer, &git_changes)?;
    let buffer = append_staged_changes(buffer, &git_changes)?;
    let buffer = append_changes_separator(buffer, &git_changes)?;
    let mut buffer = append_unstaged_changes(buffer, &git_changes)?;

    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
    write!(&mut buffer, "]")?;
    buffer.set_color(ColorSpec::new().set_reset(true))?;

    return Ok(buffer);
}

fn append_branch_name(
    mut buffer: Buffer,
    git_changes: &GitChanges,
) -> Result<Buffer, std::io::Error> {
    if let Some(bs) = &git_changes.branch_status {
        if bs.behind > 0 {
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
        } else {
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        }
    } else {
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
    }

    write!(buffer, "{}", git_changes.branch_name)?;

    return Ok(buffer);
}

fn append_branch_status(
    mut buffer: Buffer,
    git_changes: &GitChanges,
) -> Result<Buffer, std::io::Error> {
    if let Some(bs) = &git_changes.branch_status {
        if bs.ahead == 0 && bs.behind == 0 && !bs.gone {
            write!(&mut buffer, " ≡")?;
        } else if bs.ahead > 0 && bs.behind > 0 {
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;

            write!(&mut buffer, " ↑{}↓{}", bs.ahead, bs.behind)?;
        } else if bs.ahead > 0 {
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
            write!(&mut buffer, " ↑{}", bs.ahead)?;
        } else if bs.behind > 0 {
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
            write!(&mut buffer, " ↓{}", bs.behind)?;
        } else if bs.gone {
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
            write!(&mut buffer, " ×",)?;
        }
    } else {
        write!(&mut buffer, " ≡")?;
    }

    return Ok(buffer);
}

fn append_staged_changes(
    mut buffer: Buffer,
    git_changes: &GitChanges,
) -> Result<Buffer, std::io::Error> {
    if let Some(ref fc) = git_changes.staged_changes {
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        write!(
            &mut buffer,
            " +{} ~{} -{}",
            fc.added, fc.modified, fc.deleted
        )?;
    }

    return Ok(buffer);
}

fn append_changes_separator(
    mut buffer: Buffer,
    git_changes: &GitChanges,
) -> Result<Buffer, std::io::Error> {
    if git_changes.staged_changes.is_some() && git_changes.unstaged_changes.is_some() {
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        write!(&mut buffer, " |")?;
    }

    return Ok(buffer);
}

fn append_unstaged_changes(
    mut buffer: Buffer,
    git_changes: &GitChanges,
) -> Result<Buffer, std::io::Error> {
    if let Some(ref fc) = git_changes.unstaged_changes {
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
        write!(
            &mut buffer,
            " +{} ~{} -{} !",
            fc.added, fc.modified, fc.deleted
        )?;
    }

    return Ok(buffer);
}
