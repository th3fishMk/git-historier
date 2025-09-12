# Git-historier

The goal of this project is to **rebuild the history of an existing repository** in a controlled, replayable way. And re-create the entire commit history, as if the repository is being built from scratch, but preserving the evolution of files over time.
This can be useful for various purposes, such as:

- Learning how a project evolved over time.
- Testing tools that analyze commit history.
- Creating a clean, simplified version of a repository for demonstration or educational purposes.
- Generating a reproducible sequence of commits for testing CI/CD pipelines or other automation.
- Experimenting with different commit messages or structures without altering the original repository.
- Creating a "sandbox" version of a repository for safe experimentation.

## How It Works

The program works this way:

we clone a repository, but instead of copying the commits directly, we rebuild them from scratch in a new repository, following the same timeline of file changes.

To achieve this, the program performs the following steps:

### prerequisites

- A local copy of the original repository to read from.
- A writeable location to create the new repository.
- Git installed on the system to manage repositories and commits.
- Optionally, a list of commit messages to use for the new commits
  - Dates can be preserved from the original commits or randomized
  - Messages can be preserved from the original commits or randomized
  - Author information can be preserved or set to a default value
  
### What the program does

Here is a detailed breakdown of the process:

- The program reads the commit history of the original repository in chronological order.
- Once the commit history is read, it initializes a new, empty repository.
  - Here, we can set default author information if desired.
- For each commit in the original repository:
  - The program runs a checkout to the specific commit in the original repository to access the file state at that point in time.
  - It copies the files from the original repository’s commit into the new repository, excluding the `.git` directory to avoid copying the original repository's metadata.
    - If we want to change the author information, the commit message, or the date, we can do it here.
  - With the `GIT_COMMITTER_DATE` and `--date` options, we can set the commit date to match the original commit or use a new date.
  - It nows creates a new commit in the new repository
  - Repeats the process for all commits in the original repository.
- Finally, the program completes the process, resulting in a new repository that mirrors the original repository's commit history, but with new commits created from scratch, and optionally modified author information, commit messages, and dates.

## Features

- Preserve original commit dates or use new dates.
- Preserve original commit messages or use new messages.
- Preserve original author information or set to a default value.
- Option to randomize commit messages from a provided list.
- Github repository search and clone by keyword.

### Additional Features to Consider

- Option to filter commits by date range or author.
- Date validation and formatting.
- Support for different version control systems (e.g., Mercurial, SVN).

## Disclaimer

This project is intended for educational and experimental purposes only. It should not be used to misrepresent the history of a repository or to create misleading commit histories. Always respect the original authors' work and contributions, and ensure that any use of this tool complies with relevant licenses and ethical guidelines. DO NOT use this tool to create fraudulent or deceptive commit histories.
