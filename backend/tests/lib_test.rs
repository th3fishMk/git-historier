#[cfg(test)]
mod tests {
    use backend::{
        clone_repo, git_add_all, git_checkout, git_commit, git_config_user, git_get_logs,
        git_init_repo,
    };
    use std::{
        fs::{self, File},
        process::Command,
    };
    use tempfile::tempdir;
    #[test]
    fn test_init_repo() {
        let dir = tempdir().unwrap();
        let repo_path = dir.path().join("test_repo");
        assert!(!repo_path.exists());
        git_init_repo(repo_path.to_str().unwrap()).unwrap();
        assert!(repo_path.join(".git").exists());
        // Clean up
        fs::remove_dir_all(repo_path).unwrap();
    }
    #[test]
    fn test_config_user() {
        let dir = tempdir().unwrap();
        let repo_path = dir.path().join("test_repo");
        let repo_path_str = repo_path.to_str().unwrap();
        git_init_repo(repo_path_str).unwrap();
        git_config_user(repo_path_str, "Test User", "Test@example.com").unwrap();
        let output_name = Command::new("git")
            .arg("-C")
            .arg(repo_path_str)
            .arg("config")
            .arg("user.name")
            .output()
            .unwrap();
        let output_email = Command::new("git")
            .arg("-C")
            .arg(repo_path_str)
            .arg("config")
            .arg("user.email")
            .output()
            .unwrap();
        assert_eq!(
            String::from_utf8(output_name.stdout).unwrap().trim(),
            "Test User"
        );
        assert_eq!(
            String::from_utf8(output_email.stdout).unwrap().trim(),
            "Test@example.com"
        );
        // Clean up
        fs::remove_dir_all(repo_path).unwrap();
    }
    #[test]
    fn test_add_all_and_commit() {
        let dir = tempdir().unwrap();
        let repo_path = dir.path().join("test_repo");
        let repo_path_str = repo_path.to_str().unwrap();
        git_init_repo(repo_path_str).unwrap();
        fs::write(repo_path.join("file.txt"), "Hello, world!").unwrap();
        git_add_all(repo_path_str).unwrap();
        git_commit(repo_path_str, "Initial commit", None).unwrap();
        let log = git_get_logs(repo_path_str).unwrap();
        assert_eq!(log[log.len() - 1].1, "Initial commit");
        // Clean up
        fs::remove_dir_all(repo_path).unwrap();
    }
    #[test]
    fn test_commit_with_date() {
        let dir = tempdir().unwrap();
        let repo_path = dir.path().join("test_repo");
        let repo_path_str = repo_path.to_str().unwrap();
        git_init_repo(repo_path_str).unwrap();
        fs::write(repo_path.join("file.txt"), "Hello, world!").unwrap();
        git_add_all(repo_path_str).unwrap();
        let date = "2023-01-01T12:00:00";
        git_commit(repo_path_str, "Initial commit", Some(date)).unwrap();
        let output = Command::new("git")
            .arg("-C")
            .arg(repo_path_str)
            .arg("log")
            .arg("-1")
            .arg("--pretty=format:%cI")
            .output()
            .unwrap();
        let commit_date = String::from_utf8(output.stdout).unwrap();
        assert!(commit_date.starts_with("2023-01-01T12:00:00"));
        // Clean up
        fs::remove_dir_all(repo_path).unwrap();
    }
    #[test]
    fn test_get_log_multiple_commits() {
        let dir = tempdir().unwrap();
        let repo_path = dir.path().join("test_repo");
        let repo_path_str = repo_path.to_str().unwrap();
        git_init_repo(repo_path_str).unwrap();
        fs::write(repo_path.join("file1.txt"), "File 1").unwrap();
        git_add_all(repo_path_str).unwrap();
        git_commit(repo_path_str, "First commit", None).unwrap();
        fs::write(repo_path.join("file2.txt"), "File 2").unwrap();
        git_add_all(repo_path_str).unwrap();
        git_commit(repo_path_str, "Second commit", None).unwrap();
        let log = git_get_logs(repo_path_str).unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(log[0].1, "Second commit");
        assert_eq!(log[1].1, "First commit");
        // Clean up
        fs::remove_dir_all(repo_path).unwrap();
    }
    #[test]
    fn test_checkout_commit() {
        let temp = tempdir().unwrap();
        let repo_path = temp.path();
        let repo_path_str = repo_path.to_str().unwrap();

        git_init_repo(repo_path_str).unwrap();

        git_config_user(repo_path_str, "Test User", "user@example.com").unwrap();

        File::create(repo_path.join("file.txt")).unwrap();

        git_add_all(repo_path_str).unwrap();
        git_commit(repo_path_str, "Initial commit", None).unwrap();

        let log = git_get_logs(repo_path_str).unwrap();
        let commit_hash = log.last().unwrap().0.clone();

        git_checkout(repo_path_str, &commit_hash).unwrap();

        let output_head = Command::new("git")
            .args(&["-C", repo_path_str, "rev-parse", "HEAD"])
            .output()
            .unwrap();
        let head_hash = String::from_utf8(output_head.stdout)
            .unwrap()
            .trim()
            .to_string();

        assert_eq!(head_hash, commit_hash);
    }
    #[test]
    fn test_clone_repo() {
        let dir = tempdir().unwrap();
        let repo_path = dir.path().join("test_repo");
        let repo_path_str = repo_path.to_str().unwrap();

        // Use the clone_repo function to clone a public repository
        let url = "https://github.com/th3fishMk/odin-calc.git";
        let _ = clone_repo(url, repo_path_str);
        let repo_name_from_url = url.rsplit('/').next().unwrap().trim_end_matches(".git");
        assert!(repo_path.join(repo_name_from_url).exists());
    }
}
