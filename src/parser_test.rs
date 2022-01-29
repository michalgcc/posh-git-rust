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
        assert_eq!(
            crate::parser::extract_branch_name(EXAMPLE_GIT_STATUS),
            Some("main".to_string())
        );
    }

    #[test]
    fn extract_branch_returns_none_when_the_status_is_empty() {
        assert_eq!(crate::parser::extract_branch_name(""), None);
    }

    #[test]
    fn extract_branch_returns_none_when_random_string_is_used() {
        assert_eq!(
            crate::parser::extract_branch_name("This is a random string \n \n"),
            None
        );
    }

    #[test]
    fn extract_changes_to_be_commited_can_parse_proper_status() {
        let result = crate::parser::extract_changes_to_be_commited(EXAMPLE_GIT_STATUS)
            .expect("Should return");

        assert_eq!(result.added, 1);
        assert_eq!(result.deleted, 1);
        assert_eq!(result.modified, 1);
    }
}