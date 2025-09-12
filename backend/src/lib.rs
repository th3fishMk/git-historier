use std::process::Command;

// Runs a git command to initialize a new repository
pub fn init_repo(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("git").arg("init").arg(path).status()?;
    if !status.success() {
        return Err("Failed to initialize git repository".into());
    }
    Ok(())
}

// Runs a git command ti add user and email configuration
pub fn config_user(path: &str, name: &str, email: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status_name = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("config")
        .arg("user.name")
        .arg(name)
        .status()?;
    if !status_name.success() {
        return Err("Failed to set user name".into());
    }
    let status_email = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("config")
        .arg("user.email")
        .arg(email)
        .status()?;
    if !status_email.success() {
        return Err("Failed to set user email".into());
    }
    Ok(())
}

// Runs a git command to add all files, if a date is provided, set the GIT_COMMITTER_DATE and GIT_AUTHOR_DATE environment variables
pub fn add_all(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("add")
        .arg(".")
        .status()?;
    if !status.success() {
        return Err("Failed to add files".into());
    }
    Ok(())
}

// Runs a git command to commit with a message, if a date is provided, set the GIT_COMMITTER_DATE and GIT_AUTHOR_DATE environment variables
pub fn commit(
    path: &str,
    message: &str,
    date: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("git");
    cmd.arg("-C").arg(path).arg("commit").arg("-m").arg(message);
    if let Some(date) = date {
        cmd.env("GIT_COMMITTER_DATE", date)
            .env("GIT_AUTHOR_DATE", date);
    }
    let status = cmd.status()?;
    if !status.success() {
        return Err("Failed to commit".into());
    }
    Ok(())
}

// Runs a git command to get the log of commits in the repository, and returns a vector of tuples containing the commit hash and commit message
pub fn get_log(path: &str) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("log")
        .arg("--pretty=format:%H;%s")
        .output()?;
    if !output.status.success() {
        return Err("Failed to get git log".into());
    }
    let log = String::from_utf8(output.stdout)?;
    let commits = log
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.splitn(2, ';').collect();
            (
                parts[0].to_string(),
                parts.get(1).unwrap_or(&"").to_string(),
            )
        })
        .collect();
    Ok(commits)
}

// Unit tests for the functions above
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;
    #[test]
    fn test_init_repo() {
        let dir = tempdir().unwrap();
        let repo_path = dir.path().join("test_repo");
        assert!(!repo_path.exists());
        init_repo(repo_path.to_str().unwrap()).unwrap();
        assert!(repo_path.join(".git").exists());
        // Clean up
        fs::remove_dir_all(repo_path).unwrap();
    }
    #[test]
    fn test_config_user() {
        let dir = tempdir().unwrap();
        let repo_path = dir.path().join("test_repo");
        let repo_path_str = repo_path.to_str().unwrap();
        init_repo(repo_path_str).unwrap();
        config_user(repo_path_str, "Test User", "Test@example.com").unwrap();
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
        init_repo(repo_path_str).unwrap();
        fs::write(repo_path.join("file.txt"), "Hello, world!").unwrap();
        add_all(repo_path_str).unwrap();
        commit(repo_path_str, "Initial commit", None).unwrap();
        let log = get_log(repo_path_str).unwrap();
        assert_eq!(log[log.len() - 1].1, "Initial commit");
        // Clean up
        fs::remove_dir_all(repo_path).unwrap();
    }
    fn test_commit_with_date() {
        let dir = tempdir().unwrap();
        let repo_path = dir.path().join("test_repo");
        let repo_path_str = repo_path.to_str().unwrap();
        init_repo(repo_path_str).unwrap();
        fs::write(repo_path.join("file.txt"), "Hello, world!").unwrap();
        add_all(repo_path_str).unwrap();
        let date = "2023-01-01T12:00:00";
        commit(repo_path_str, "Initial commit", Some(date)).unwrap();
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
        init_repo(repo_path_str).unwrap();
        fs::write(repo_path.join("file1.txt"), "File 1").unwrap();
        add_all(repo_path_str).unwrap();
        commit(repo_path_str, "First commit", None).unwrap();
        fs::write(repo_path.join("file2.txt"), "File 2").unwrap();
        add_all(repo_path_str).unwrap();
        commit(repo_path_str, "Second commit", None).unwrap();
        let log = get_log(repo_path_str).unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(log[0].1, "Second commit");
        assert_eq!(log[1].1, "First commit");
        // Clean up
        fs::remove_dir_all(repo_path).unwrap();
    }
}
