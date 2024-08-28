use std::{
    path::Path,
    process::Command,
};
fn git_push(repo_path: &str, remote: &str, branch: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo_path = Path::new(repo_path);

    if !repo_path.exists() || !repo_path.is_dir() {
        return Err(format!("Invalid repository path: {}", repo_path.display()).into());
    }

    // Check if there are changes to push
    let status_output = Command::new("git")
        .current_dir(repo_path)
        .args(&["status", "--porcelain"])
        .output()?;

    if status_output.stdout.is_empty() {
        println!("No changes to push.");
        return Ok(());
    }

    // Show what's being pushed
    println!("Changes to be pushed:");
    let diff_output = Command::new("git")
        .current_dir(repo_path)
        .args(&["add", "./*"])
        .output();
    match diff_output {
        Ok(_op) => {
            let commit=Command::new("git").current_dir(repo_path).args(&["commit","-m","committed by code"]).output()?;
            let push_output = Command::new("git")
                .current_dir(repo_path)
                .args(&["push", "-v", remote, branch])
                .output()?;

            println!("Git push output:");
            println!("stdout: {}", String::from_utf8_lossy(&push_output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&push_output.stderr));

            if push_output.status.success() {
                println!("Git push command executed successfully.");
                Ok(())
            } else {
                Err(format!(
                    "Git push failed: {}",
                    String::from_utf8_lossy(&push_output.stderr)
                )
                .into())
            }
        }
        Err(error) => {
            println!("37:::>{}", error);
            Err(format!("37:::>{}", error).into())
        }
    }
    // Perform the push
}
fn main() {
    // let repo_path = "/home/rahul/Desktop/Node_code/git_wrapper";
    let repo_path = "/home/rahul/Desktop/Rust_code/git_wrapper_rust";
    let remote = "origin";
    let branch = "main";

    let op = git_push(repo_path, remote, branch);
    println!("{:?}", op);
}
