use backend;

fn main() {
    let repo_path_str = "https://github.com/th3fishMk/odin-curriculum.git";
    let user_name = "Your Name";
    let user_email = "user@exa/mple.com";
    let dest_path = "/home/fish/Repos/fish/new-repo/";

    backend::start_process(
        repo_path_str,
        "/tmp/empty_dir",
        dest_path,
        user_name,
        user_email,
    );
    println!("Process completed successfully.");
}
