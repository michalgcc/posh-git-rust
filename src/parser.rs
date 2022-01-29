const ON_BRANCH_STRING: &str = "On branch ";
const EMPTY_STRING: &str = "";
const NEW_LINE_STRING: &str = "\n";
const CHANGES_TO_BE_COMMITED_STRING: &str = "Changes to be committed:";
const CHANGES_NOT_STAGED_FOR_COMMIT: &str = "Changes not staged for commit:";

#[derive(Debug)]
pub struct FilesChanges {
    pub added: i32,
    pub deleted: i32,
    pub modified: i32,
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

    // TODO Parse Untracked files and add them as added

    if changes_not_staged_for_commit_lines.len() == 0 {
        return None;
    }

    extract_files_changes(changes_not_staged_for_commit_lines)
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

    let changes_to_be_commited_start = input_lines.skip_while(|&l| l.find(skip_while) == None);
    let changes_to_be_commited: Vec<&str> = changes_to_be_commited_start
        .take_while(|&l| l.len() > 2)
        .collect();
    changes_to_be_commited
}
