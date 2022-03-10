#[cfg(test)]
mod formatter_test {
    use crate::formatter::format_git_status_prompt_buffer;
    use crate::parser::BranchStatus;
    use crate::parser::FileChanges;
    use crate::parser::GitChanges;
    use termcolor::ColorChoice;

    #[test]
    fn format_no_changes() {
        let git_changes = GitChanges {
            branch_name: "main".to_string(),
            staged_changes: None,
            unstaged_changes: None,
            branch_status: None,
        };
        let no_color_result =
            format_git_status_prompt_buffer(&git_changes, ColorChoice::Never).unwrap();
        let no_color_result = std::str::from_utf8(no_color_result.as_slice());

        let ansi_result =
            format_git_status_prompt_buffer(&git_changes, ColorChoice::AlwaysAnsi).unwrap();
        let ansi_result = std::str::from_utf8(ansi_result.as_slice());

        assert_eq!(no_color_result.unwrap(), "[main ≡]".to_string());
        assert_eq!(
            ansi_result.unwrap(),
            "\u{1}\u{1b}[0m\u{1b}[33m\u{2}[\u{1}\u{1b}[0m\u{1b}[36m\u{2}main ≡\u{1}\u{1b}[0m\u{1b}[33m\u{2}]\u{1}\u{1b}[0m\u{2}".to_string()
        );
    }

    #[test]
    fn format_incoming_changes_just_behind() {
        let git_changes = GitChanges {
            branch_name: "main".to_string(),
            staged_changes: None,
            unstaged_changes: None,
            branch_status: Some(BranchStatus {
                ahead: 0,
                behind: 1,
                gone: false,
            }),
        };
        let no_color_result =
            format_git_status_prompt_buffer(&git_changes, ColorChoice::Never).unwrap();
        let no_color_result = std::str::from_utf8(no_color_result.as_slice());

        let ansi_result =
            format_git_status_prompt_buffer(&git_changes, ColorChoice::AlwaysAnsi).unwrap();
        let ansi_result = std::str::from_utf8(ansi_result.as_slice());

        assert_eq!(no_color_result.unwrap(), "[main ↓1]".to_string());
        assert_eq!(
            ansi_result.unwrap(),
            "\u{1}\u{1b}[0m\u{1b}[33m\u{2}[\u{1}\u{1b}[0m\u{1b}[31m\u{2}main\u{1}\u{1b}[0m\u{1b}[31m\u{2} ↓1\u{1}\u{1b}[0m\u{1b}[33m\u{2}]\u{1}\u{1b}[0m\u{2}".to_string()
        );
    }

    #[test]
    fn format_incoming_changes_just_ahead() {
        let git_changes = GitChanges {
            branch_name: "main".to_string(),
            staged_changes: None,
            unstaged_changes: None,
            branch_status: Some(BranchStatus {
                ahead: 1,
                behind: 0,
                gone: false,
            }),
        };
        let no_color_result =
            format_git_status_prompt_buffer(&git_changes, ColorChoice::Never).unwrap();
        let no_color_result = std::str::from_utf8(no_color_result.as_slice());

        let ansi_result =
            format_git_status_prompt_buffer(&git_changes, ColorChoice::AlwaysAnsi).unwrap();
        let ansi_result = std::str::from_utf8(ansi_result.as_slice());

        assert_eq!(no_color_result.unwrap(), "[main ↑1]".to_string());
        assert_eq!(
            ansi_result.unwrap(),
            "\u{1}\u{1b}[0m\u{1b}[33m\u{2}[\u{1}\u{1b}[0m\u{1b}[36m\u{2}main\u{1}\u{1b}[0m\u{1b}[32m\u{2} ↑1\u{1}\u{1b}[0m\u{1b}[33m\u{2}]\u{1}\u{1b}[0m\u{2}".to_string()
        );
    }

    #[test]
    fn format_incoming_changes_behind_and_ahead() {
        let git_changes = GitChanges {
            branch_name: "main".to_string(),
            staged_changes: None,
            unstaged_changes: None,
            branch_status: Some(BranchStatus {
                ahead: 1,
                behind: 1,
                gone: false,
            }),
        };
        let no_color_result =
            format_git_status_prompt_buffer(&git_changes, ColorChoice::Never).unwrap();
        let no_color_result = std::str::from_utf8(no_color_result.as_slice());

        let ansi_result =
            format_git_status_prompt_buffer(&git_changes, ColorChoice::AlwaysAnsi).unwrap();
        let ansi_result = std::str::from_utf8(ansi_result.as_slice());

        assert_eq!(no_color_result.unwrap(), "[main ↑1↓1]".to_string());
        assert_eq!(
            ansi_result.unwrap(),
            "\u{1}\u{1b}[0m\u{1b}[33m\u{2}[\u{1}\u{1b}[0m\u{1b}[31m\u{2}main\u{1}\u{1b}[0m\u{1b}[33m\u{2} ↑1↓1\u{1}\u{1b}[0m\u{1b}[33m\u{2}]\u{1}\u{1b}[0m\u{2}".to_string()
        );
    }

    #[test]
    fn format_staged_changes() {
        let git_changes = GitChanges {
            branch_name: "main".to_string(),
            staged_changes: Some(FileChanges {
                added: 1,
                deleted: 1,
                modified: 1,
            }),
            unstaged_changes: None,
            branch_status: Some(BranchStatus {
                ahead: 0,
                behind: 0,
                gone: false,
            }),
        };
        let no_color_result =
            format_git_status_prompt_buffer(&git_changes, ColorChoice::Never).unwrap();
        let no_color_result = std::str::from_utf8(no_color_result.as_slice());

        let ansi_result =
            format_git_status_prompt_buffer(&git_changes, ColorChoice::AlwaysAnsi).unwrap();
        let ansi_result = std::str::from_utf8(ansi_result.as_slice());

        assert_eq!(no_color_result.unwrap(), "[main ≡ +1 ~1 -1]".to_string());
        assert_eq!(
            ansi_result.unwrap(),
            "\u{1}\u{1b}[0m\u{1b}[33m\u{2}[\u{1}\u{1b}[0m\u{1b}[36m\u{2}main ≡\u{1}\u{1b}[0m\u{1b}[32m\u{2} +1 ~1 -1\u{1}\u{1b}[0m\u{1b}[33m\u{2}]\u{1}\u{1b}[0m\u{2}".to_string()
        );
        println!("{}", ansi_result.unwrap());
    }

    #[test]
    fn format_unstaged_changes() {
        let git_changes = GitChanges {
            branch_name: "main".to_string(),
            staged_changes: None,
            unstaged_changes: Some(FileChanges {
                added: 1,
                deleted: 1,
                modified: 1,
            }),
            branch_status: Some(BranchStatus {
                ahead: 0,
                behind: 0,
                gone: false,
            }),
        };
        let no_color_result =
            format_git_status_prompt_buffer(&git_changes, ColorChoice::Never).unwrap();
        let no_color_result = std::str::from_utf8(no_color_result.as_slice());

        let ansi_result =
            format_git_status_prompt_buffer(&git_changes, ColorChoice::AlwaysAnsi).unwrap();
        let ansi_result = std::str::from_utf8(ansi_result.as_slice());

        assert_eq!(no_color_result.unwrap(), "[main ≡ +1 ~1 -1 !]".to_string());
        assert_eq!(
            ansi_result.unwrap(),
            "\u{1}\u{1b}[0m\u{1b}[33m\u{2}[\u{1}\u{1b}[0m\u{1b}[36m\u{2}main ≡\u{1}\u{1b}[0m\u{1b}[31m\u{2} +1 ~1 -1 !\u{1}\u{1b}[0m\u{1b}[33m\u{2}]\u{1}\u{1b}[0m\u{2}".to_string()
        );
        println!("{}", ansi_result.unwrap());
    }

    #[test]
    fn format_staged_and_unstaged_changes() {
        let git_changes = GitChanges {
            branch_name: "main".to_string(),
            staged_changes: Some(FileChanges {
                added: 1,
                deleted: 1,
                modified: 1,
            }),
            unstaged_changes: Some(FileChanges {
                added: 1,
                deleted: 1,
                modified: 1,
            }),
            branch_status: Some(BranchStatus {
                ahead: 0,
                behind: 0,
                gone: false,
            }),
        };
        let no_color_result =
            format_git_status_prompt_buffer(&git_changes, ColorChoice::Never).unwrap();
        let no_color_result = std::str::from_utf8(no_color_result.as_slice());

        let ansi_result =
            format_git_status_prompt_buffer(&git_changes, ColorChoice::AlwaysAnsi).unwrap();
        let ansi_result = std::str::from_utf8(ansi_result.as_slice());

        assert_eq!(
            no_color_result.unwrap(),
            "[main ≡ +1 ~1 -1 | +1 ~1 -1 !]".to_string()
        );
        assert_eq!(
            ansi_result.unwrap(),
            "\u{1}\u{1b}[0m\u{1b}[33m\u{2}[\u{1}\u{1b}[0m\u{1b}[36m\u{2}main ≡\u{1}\u{1b}[0m\u{1b}[32m\u{2} +1 ~1 -1\u{1}\u{1b}[0m\u{1b}[33m\u{2} |\u{1}\u{1b}[0m\u{1b}[31m\u{2} +1 ~1 -1 !\u{1}\u{1b}[0m\u{1b}[33m\u{2}]\u{1}\u{1b}[0m\u{2}" 
                .to_string()
        );
        println!("{}", ansi_result.unwrap());
    }

    #[test]
    fn format_incoming_and_outgoing_staged_and_unstaged_changes() {
        let git_changes = GitChanges {
            branch_name: "main".to_string(),
            staged_changes: Some(FileChanges {
                added: 1,
                deleted: 1,
                modified: 1,
            }),
            unstaged_changes: Some(FileChanges {
                added: 1,
                deleted: 1,
                modified: 1,
            }),
            branch_status: Some(BranchStatus {
                ahead: 1,
                behind: 1,
                gone: false,
            }),
        };
        let no_color_result =
            format_git_status_prompt_buffer(&git_changes, ColorChoice::Never).unwrap();
        let no_color_result = std::str::from_utf8(no_color_result.as_slice());

        let ansi_result =
            format_git_status_prompt_buffer(&git_changes, ColorChoice::AlwaysAnsi).unwrap();
        let ansi_result = std::str::from_utf8(ansi_result.as_slice());

        assert_eq!(
            no_color_result.unwrap(),
            "[main ↑1↓1 +1 ~1 -1 | +1 ~1 -1 !]".to_string()
        );
        assert_eq!(
            ansi_result.unwrap(),
            "\u{1}\u{1b}[0m\u{1b}[33m\u{2}[\u{1}\u{1b}[0m\u{1b}[31m\u{2}main\u{1}\u{1b}[0m\u{1b}[33m\u{2} ↑1↓1\u{1}\u{1b}[0m\u{1b}[32m\u{2} +1 ~1 -1\u{1}\u{1b}[0m\u{1b}[33m\u{2} |\u{1}\u{1b}[0m\u{1b}[31m\u{2} +1 ~1 -1 !\u{1}\u{1b}[0m\u{1b}[33m\u{2}]\u{1}\u{1b}[0m\u{2}"
                .to_string()
        );
        println!("{}", ansi_result.unwrap());
    }

    #[test]
    fn format_gone() {
        let git_changes = GitChanges {
            branch_name: "main".to_string(),
            staged_changes: None,
            unstaged_changes: None,
            branch_status: Some(BranchStatus {
                ahead: 0,
                behind: 0,
                gone: true,
            }),
        };
        let no_color_result =
            format_git_status_prompt_buffer(&git_changes, ColorChoice::Never).unwrap();
        let no_color_result = std::str::from_utf8(no_color_result.as_slice());

        let ansi_result =
            format_git_status_prompt_buffer(&git_changes, ColorChoice::AlwaysAnsi).unwrap();
        let ansi_result = std::str::from_utf8(ansi_result.as_slice());

        assert_eq!(no_color_result.unwrap(), "[main ×]".to_string());
        assert_eq!(
            ansi_result.unwrap(),
            "\u{1}\u{1b}[0m\u{1b}[33m\u{2}[\u{1}\u{1b}[0m\u{1b}[36m\u{2}main\u{1}\u{1b}[0m\u{1b}[31m\u{2} ×\u{1}\u{1b}[0m\u{1b}[33m\u{2}]\u{1}\u{1b}[0m\u{2}".to_string()
        );
        println!("{}", ansi_result.unwrap());
    }
}
