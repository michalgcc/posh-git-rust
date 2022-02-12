#[cfg(test)]
mod parser_test {

    const EXAMPLE_GIT_STATUS: &str = "\
On branch main
Your branch is behind 'origin/main' by 21 commits, and can be fast-forwarded.
  (use \"git pull\" to update your local branch)

Changes to be committed:
  (use \"git restore --staged <file>...\" to unstage)
        deleted:    release-and-copy-to-bin.sh
        new file:   test.txt
        modified:   src/main.rs


Changes not staged for commit:
  (use \"git add <file>...\" to update what will be committed)
  (use \"git restore <file>...\" to discard changes in working directory)
        deleted:    release-and-copy-to-bin-test.sh
        modified:   src/Services/a.csproj
        modified:   src/Services/b.cs
        modified:   src/Services/c.cs
        modified:   src/Services/d.cs
        modified:   src/Services/e.cs

Untracked files:
  (use \"git add <file>...\" to include in what will be committed)
        test2.txt
";

    #[test]
    fn extract_branch_can_parse_proper_status() {
        let result = crate::parser::extract_git_changes(EXAMPLE_GIT_STATUS).unwrap();

        assert_eq!(result.branch_name, "main".to_string());
    }

    #[test]
    fn extract_branch_returns_none_when_the_status_is_empty() {
        let result = crate::parser::extract_git_changes("");

        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn extract_branch_returns_none_when_random_string_is_used() {
        let result = crate::parser::extract_git_changes("This is a random string \n \n");
        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn extract_staged_changes_can_parse_proper_status() {
        let result = crate::parser::extract_git_changes(EXAMPLE_GIT_STATUS).expect("Should return");
        let staged_changes = result.staged_changes.expect("Should have staged changes");

        assert_eq!(staged_changes.added, 1);
        assert_eq!(staged_changes.deleted, 1);
        assert_eq!(staged_changes.modified, 1);
    }

    #[test]
    fn extract_unstaged_changes_can_parse_proper_status() {
        let result = crate::parser::extract_git_changes(EXAMPLE_GIT_STATUS).expect("Should return");
        let unstaged_changes = result
            .unstaged_changes
            .expect("Should have unstaged changes");

        assert_eq!(unstaged_changes.added, 1);
        assert_eq!(unstaged_changes.deleted, 1);
        assert_eq!(unstaged_changes.modified, 5);
    }

    #[test]
    fn extract_branch_status_can_parse_proper_status() {
        let result = crate::parser::extract_git_changes(EXAMPLE_GIT_STATUS).expect("Should return");
        let branch_status = result.branch_status.expect("Should have branch changes");

        // Branch status
        assert_eq!(branch_status.ahead, 0);
        assert_eq!(branch_status.behind, 21);
        assert_eq!(branch_status.gone, false);
    }

    #[test]
    fn extract_branch_status_can_parse_behind_status() {
        let result = crate::parser::extract_git_changes(
            "On branch main\n Your branch is behind 'origin/main' by 2 commits, and can be fast forwarded.",
        )
        .expect("Should return");
        let branch_status = result.branch_status.expect("Should have branch changes");

        assert_eq!(branch_status.ahead, 0);
        assert_eq!(branch_status.behind, 2);
        assert_eq!(branch_status.gone, false);
    }

    #[test]
    fn extract_branch_status_can_parse_ahead_status() {
        let result = crate::parser::extract_git_changes(
            "On branch main\n Your branch is ahead of 'origin/main' by 1 commit.",
        )
        .expect("Should return");
        let branch_status = result.branch_status.expect("Should have branch changes");

        assert_eq!(branch_status.ahead, 1);
        assert_eq!(branch_status.behind, 0);
        assert_eq!(branch_status.gone, false);
    }

    #[test]
    fn extract_branch_status_can_parse_diverged_status() {
        let result = crate::parser::extract_git_changes(
            "On branch main\n Your branch and 'origin/master' have diverged,\nand have 1 and 2 different commits each, respectively.",
        )
        .expect("Should return");
        let branch_status = result.branch_status.expect("Should have branch changes");

        assert_eq!(branch_status.ahead, 1);
        assert_eq!(branch_status.behind, 2);
        assert_eq!(branch_status.gone, false);
    }

    #[test]
    fn extract_branch_status_can_parse_gone_status() {
        let result = crate::parser::extract_git_changes(
            "On branch main\n Your branch is based on 'origin/main', but the upstream is gone.",
        )
        .expect("Should return");
        let branch_status = result.branch_status.expect("Should have branch changes");

        assert_eq!(branch_status.ahead, 0);
        assert_eq!(branch_status.behind, 0);
        assert_eq!(branch_status.gone, true);
    }
}
