const ON_BRANCH_STRING: &str = "On branch ";
const EMPTY_STRING: &str = "";
const NEW_LINE_STRING: &str = "\n";

pub fn extract_branch_name(input: &str) -> Option<String> {
    let branch_positon = input.find(ON_BRANCH_STRING)? + ON_BRANCH_STRING.len();
    let parsed_line = input.split_at(branch_positon).1;
    let new_line_position = parsed_line.find(NEW_LINE_STRING)?;
    let parsed_line = parsed_line.split_at(new_line_position).0;

    let result = parsed_line.replace(ON_BRANCH_STRING, EMPTY_STRING);

    Some(result)
}
