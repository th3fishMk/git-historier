use std::process::Command;

// Runs a git command to initialize a new repository
pub fn git_init_repo(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("git").arg("init").arg(path).status()?;
    if !status.success() {
        return Err("Failed to initialize git repository".into());
    }
    Ok(())
}

// Runs a git command ti add user and email configuration
pub fn git_config_user(
    path: &str,
    name: &str,
    email: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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
pub fn git_add_all(path: &str) -> Result<(), Box<dyn std::error::Error>> {
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
pub fn git_commit(
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

// Runs a git command to get the log of commits in the repository, and returns a vector of tuples containing the commit hash, commit message, and the commit date
pub fn git_get_logs(
    path: &str,
) -> Result<Vec<(String, String, String)>, Box<dyn std::error::Error>> {
    // Attempt to read the git log, if it fails return an error
    // print the given path for debugging
    println!("Getting git log for path: {}", path);

    // Print everything in the directory for debugging, in a new line
    println!(
        "Directory contents: {:?}",
        std::fs::read_dir(path)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .unwrap()
    );

    

    let output = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("log")
        .arg("--pretty=format:%H;%s;%cI")
        .output()?;
    if !output.status.success() {
        return Err("Failed to get git log in this line right here".into());
    }
    let log = String::from_utf8(output.stdout)?;
    let commits = log
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.splitn(3, ';').collect();
            (
                parts.get(0).unwrap_or(&"").to_string(),
                parts.get(1).unwrap_or(&"").to_string(),
                parts.get(2).unwrap_or(&"").to_string(),
            )
        })
        .collect();
    Ok(commits)
}

// Runs a git command to 'checkout' a specific commit by its hash
pub fn git_checkout(path: &str, commit_hash: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("checkout")
        .arg(commit_hash)
        .status()?;
    if !status.success() {
        return Err("Failed to checkout commit".into());
    }
    Ok(())
}

// Copy the contents of one directory to another
pub fn copy_dir_all(
    src: &std::path::Path,
    dst: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    if !dst.exists() {
        std::fs::create_dir_all(dst)?;
    }
    // Log the source and destination paths
    println!("Copying from {:?} to {:?}", src, dst);

    for entry in std::fs::read_dir(src)? {
        // Log each entry being processed
        println!("Processing entry: {:?}", entry);

        let entry = entry?;
        let file_type = entry.file_type()?;
        let dest_path = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_all(&entry.path(), &dest_path)?;
        } else {
            std::fs::copy(&entry.path(), &dest_path)?;
        }
        // Sleep the thread for 10 milliseconds to avoid overwhelming the system
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    Ok(())
}

// Verify if a directory is empty
pub fn is_dir_empty(path: &std::path::Path) -> Result<bool, Box<dyn std::error::Error>> {
    let mut entries = std::fs::read_dir(path)?;
    Ok(entries.next().is_none())
}

// Clone a git repository from a given URL to a specified path
pub fn clone_repo(url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Check if the destination path exists and is empty
    let dest_path = std::path::Path::new(path);
    if dest_path.exists() {
        if !is_dir_empty(dest_path)? {
            return Err("Destination path is not empty".into());
        }
    } else {
        std::fs::create_dir_all(dest_path)?;
    }

    let status = Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(path)
        .status()?;
    if !status.success() {
        return Err("Failed to clone git repository".into());
    }
    Ok(())
}

// Start the full process
pub fn start_process(
    repo_url_or_path: &str,
    temp_empty_dir: &str,
    dest_path: &str,
    user_name: &str,
    user_email: &str,
) {
    // Step 1: Check if the repo_url_or_path is a URL or a local path
    let is_url = repo_url_or_path.starts_with("http://")
        || repo_url_or_path.starts_with("https://")
        || repo_url_or_path.starts_with("git@");
    let repo_path_str = if is_url {
        // Stop the thread for 1 second to ensure any previous operations are complete
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Gets the path to the current directory
        // Use the tempfile crate to create a temporary directory

        // It's a URL, clone the repository to a temporary directory
        let temp_repo_dir = std::path::Path::new(temp_empty_dir).join("cloned_repo");
        let temp_repo_path = temp_repo_dir.to_str().unwrap().to_owned();
        clone_repo(repo_url_or_path, &temp_repo_path).expect("Failed to clone repository");
        temp_repo_path
    } else {
        // It's a local path, use it directly
        repo_url_or_path.to_owned()
    };
    // Check if the cloned or provided directory contains a .git folder
    if !std::path::Path::new(&repo_path_str).join(".git").exists() {
        // Logs the provided path
        println!("Provided path: {}", repo_path_str);
        //

        // log the directory contents for debugging

        println!(
            "Directory contents: {:?}",
            std::fs::read_dir(&repo_path_str)
                .unwrap()
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, std::io::Error>>()
                .unwrap()
        );
        panic!("The provided path is not a valid git repository");
    }
    // Step 2: I initialize the repository
    git_init_repo(&repo_path_str).expect("Failed to initialize repository");
    // Step 3: I configure the user name and email
    git_config_user(&repo_path_str, user_name, user_email).expect("Failed to configure user");
    // Step 4: get the log of commits
    let log = git_get_logs(&repo_path_str).expect("Failed to get git log");
    // Step 5: For each commit in the log, checkout the commit, copy the contents to the destination directory, and commit the changes
    for (commit_hash, commit_message, _commit_date) in log.iter().rev() {
        git_checkout(&repo_path_str, commit_hash).expect("Failed to checkout commit");
        copy_dir_all(
            std::path::Path::new(&repo_path_str),
            std::path::Path::new(dest_path),
        )
        .expect("Failed to copy directory");
        //

        git_add_all(dest_path).expect("Failed to add files");
        git_commit(dest_path, commit_message, Some(_commit_date.as_str()))
            .expect("Failed to commit changes");
        // log progress
        println!("Processed commit: {} - {}", commit_hash, commit_message);
        // if this was the last commit log a different message
        if commit_hash == &log.first().unwrap().0 {
            println!("All commits have been processed successfully.");
        }
        // Sleep the thread for 100 milliseconds to avoid overwhelming the system
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
