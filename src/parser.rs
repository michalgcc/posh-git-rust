const ON_BRANCH_STRING: &str = "On branch ";
const EMPTY_STRING: &str = "";
const NEW_LINE_STRING: &str = "\n";
const CHANGES_TO_BE_COMMITED_STRING: &str = "Changes to be committed:";
const CHANGES_NOT_STAGED_FOR_COMMIT: &str = "Changes not staged for commit:";
const UNTRACKED_FILES_SECOND_HEADER: &str =
    "(use \"git add <file>...\" to include in what will be committed)";

#[derive(Debug)]
pub struct FilesChanges {
    pub added: i32,
    pub deleted: i32,
    pub modified: i32,
}

pub struct BranchStatus {
    pub ahead: i32,
    pub behind: i32,
    pub gone: bool,
}

pub fn extract_branch_name(input: &str) -> Option<String> {
    let branch_positon = input.find(ON_BRANCH_STRING)? + ON_BRANCH_STRING.len();
    let parsed_line = input.split_at(branch_positon).1;
    let new_line_position = parsed_line.find(NEW_LINE_STRING)?;
    let parsed_line = parsed_line.split_at(new_line_position).0;

    let result = parsed_line.replace(ON_BRANCH_STRING, EMPTY_STRING);

    Some(result)
}

pub fn extract_changes_to_be_commited(input: &str) -> Option<FilesChanges> {
    let changes_to_be_commited_lines = extract_relevant_lines(input, CHANGES_TO_BE_COMMITED_STRING);

    if changes_to_be_commited_lines.len() == 0 {
        return None;
    }

    extract_files_changes(changes_to_be_commited_lines)
}

pub fn extract_unstaged_changes(input: &str) -> Option<FilesChanges> {
    let changes_not_staged_for_commit_lines =
        extract_relevant_lines(input, CHANGES_NOT_STAGED_FOR_COMMIT);

    let untracked_files_lines = extract_relevant_lines(input, UNTRACKED_FILES_SECOND_HEADER);

    if changes_not_staged_for_commit_lines.len() == 0 && untracked_files_lines.len() == 0 {
        return None;
    }

    let partial_result = extract_files_changes(changes_not_staged_for_commit_lines)?;

    Some(FilesChanges {
        added: untracked_files_lines.len() as i32,
        deleted: partial_result.deleted,
        modified: partial_result.modified,
    })
}

pub fn extract_branch_status(input: &str) -> Option<BranchStatus> {
    let first_few_input_lines = input.lines().take(3);

    let diverged_lines: Vec<&str> = first_few_input_lines
        .filter(|&l| l.find(" different commits each, respectively.").is_some())
        .collect();

    if diverged_lines.len() > 0 {
        const AND_HAVE: &str = "and have ";

        let diverged_line = diverged_lines.first().unwrap();
        let ahead_start_index = diverged_line.find(AND_HAVE)? + AND_HAVE.len();

        let ahead_raw_string: String = diverged_line
            .chars()
            .skip(ahead_start_index)
            .take_while(|&c| c != ' ')
            .collect();

        let behind_start_index = ahead_start_index + ahead_raw_string.len() + " and ".len();

        let behind_raw_string: String = diverged_line
            .chars()
            .skip(behind_start_index)
            .take_while(|&c| c != ' ')
            .collect();

        let ahead = parse_to_i32_as_option(&ahead_raw_string)?;
        let behind = parse_to_i32_as_option(&behind_raw_string)?;

        return Some(BranchStatus {
            ahead: ahead,
            behind: behind,
            gone: false,
        });
    }

    let first_few_input_lines = input.lines().take(3);

    let upstream_is_gone_lines: Vec<&str> = first_few_input_lines
        .filter(|&l| l.find(", but the upstream is gone.").is_some())
        .collect();

    if upstream_is_gone_lines.len() > 0 {
        return Some(BranchStatus {
            ahead: 0,
            behind: 0,
            gone: true,
        });
    }

    let first_few_input_lines = input.lines().take(3);

    let ahead_lines: Vec<&str> = first_few_input_lines
        .filter(|&l| l.find("Your branch is ahead of ").is_some())
        .collect();

    let ahead_lines_number_of_commits = extract_number_of_commits(ahead_lines);
    if ahead_lines_number_of_commits.is_some() {
        return Some(BranchStatus {
            ahead: ahead_lines_number_of_commits.unwrap(),
            behind: 0,
            gone: false,
        });
    }

    let first_few_input_lines = input.lines().take(3);

    let behind_line: Vec<&str> = first_few_input_lines
        .filter(|&l| l.find("Your branch is behind ").is_some())
        .collect();

    let behind_lines_number_of_commits = extract_number_of_commits(behind_line);
    if behind_lines_number_of_commits.is_some() {
        return Some(BranchStatus {
            ahead: 0,
            behind: behind_lines_number_of_commits.unwrap(),
            gone: false,
        });
    }

    None
}

fn extract_number_of_commits(input: Vec<&str>) -> Option<i32> {
    if let Some(ahead_line) = input.first() {
        let by_index = ahead_line.find("by ")? + 3;
        let commits_index = ahead_line.find(" commit")?;

        let raw_result: String = ahead_line
            .chars()
            .skip(by_index)
            .take(commits_index - by_index)
            .collect();

        return parse_to_i32_as_option(&raw_result);
    }
    None
}

fn parse_to_i32_as_option(input: &str) -> Option<i32> {
    match input.parse::<i32>() {
        Ok(i) => return Some(i),
        _ => return None,
    }
}

fn extract_files_changes(input: Vec<&str>) -> Option<FilesChanges> {
    let mut added = 0;
    let mut deleted = 0;
    let mut modified = 0;

    for elem in input {
        let trimmed = elem.trim();
        if trimmed.starts_with("new file:") {
            added = added + 1;
        }
        if trimmed.starts_with("deleted:") {
            deleted = deleted + 1;
        }
        if trimmed.starts_with("modified:") {
            modified = modified + 1;
        }
    }

    Some(FilesChanges {
        added: added,
        deleted: deleted,
        modified: modified,
    })
}

fn extract_relevant_lines<'a>(input: &'a str, skip_while: &str) -> Vec<&'a str> {
    let input_lines = input.lines();

    let changes_to_be_commited_start = input_lines.skip_while(|&l| l.find(skip_while).is_none());
    let changes_to_be_commited: Vec<&str> = changes_to_be_commited_start
        .take_while(|&l| l.len() > 2)
        .skip(1)
        .collect();
    changes_to_be_commited
}
