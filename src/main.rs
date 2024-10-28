use std::{path::Path, process::Command};

pub fn git_status(repo_path: &str) -> Result<std::process::Output, std::io::Error> {
    // let repo_path = Path::new(repo_path);

    // if !repo_path.exists() || !repo_path.is_dir() {
    //     return Err(format!("Invalid repository path: {}", repo_path.display()).into());
    // }
    let mut binding = Command::new("git");
    let command = binding.current_dir(repo_path);
    let status_output: Result<std::process::Output, std::io::Error> =
        command.args(&["status", "--porcelain"]).output();
    status_output
    // match status_output {
    //     Ok(status_output) => {
    //         if status_output.stdout.is_empty() {
    //             println!("No changes to push.");
    //             return Ok(());
    //         }
    //         println!("Git Status::>{:?}", status_output);
    //         Ok(())
    //     }
    //     Err(error) => {
    //         eprint!("{}", error);
    //         Err(format!("{}", error).into())
    //     }
    // }
}

pub fn git_pull(
    repo_path: &str,
    remote: &str,
    branch: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let repo_path = Path::new(repo_path);

    if !repo_path.exists() || !repo_path.is_dir() {
        return Err(format!("Invalid repository path: {}", repo_path.display()).into());
    }

    let pull_op = Command::new("git")
        .current_dir(repo_path)
        .args(&["pull", "-v", remote, branch])
        .output();
    match pull_op {
        Ok(op) => {
            if op.status.success() {
                println!("Successfully pulled from remote.");
            } else {
                eprintln!(
                    "Error pulling from remote: {}",
                    String::from_utf8_lossy(&op.stderr)
                );
                return Err(
                    format!("Git pull failed: {}", String::from_utf8_lossy(&op.stderr)).into(),
                );
            }
        }
        Err(error) => {
            eprintln!("{}", error);
            return Err(error.into());
        }
    }

    Ok(())
}

pub fn git_push(
    repo_path: &str,
    remote: &str,
    branch: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let repo_path = Path::new(repo_path);

    if !repo_path.exists() || !repo_path.is_dir() {
        return Err(format!("Invalid repository path: {}", repo_path.display()).into());
    }

    // Step 1: Pull latest changes from remote
    match git_pull(repo_path.to_str().unwrap(), remote, branch) {
        Ok(_) => {
            println!("Successfully pulled changes from remote.");
        }
        Err(err) => {
            println!("Error pulling changes: {:?}", err);
            return Err(err);
        }
    }

    // Step 2: Check status of repository before pushing
    let status_output = git_status(repo_path.to_str().unwrap());
    match status_output {
        Ok(status_output) => {
            if status_output.stdout.is_empty() {
                println!("No changes to push.");
                return Ok(());
            }
        }
        Err(error) => {
            return Err(format!("Git status error: {}", error).into());
        }
    }

    // Step 3: Stage and commit local changes
    println!("Changes to be pushed:");
    let add_output = Command::new("git")
        .current_dir(repo_path)
        .args(&["add", "."])
        .output();

    match add_output {
        Ok(op) => {
            if !op.status.success() {
                return Err(
                    format!("Git add failed: {}", String::from_utf8_lossy(&op.stderr)).into(),
                );
            }

            let commit_output = Command::new("git")
                .current_dir(repo_path)
                .args(&["commit", "-m", "Committed from code generator"])
                .output();

            match commit_output {
                Ok(commit_op) => {
                    if !commit_op.status.success() {
                        println!(
                            "Nothing to commit or commit failed: {}",
                            String::from_utf8_lossy(&commit_op.stderr)
                        );
                    } else {
                        println!("Commit successful.");
                    }
                }
                Err(error) => {
                    return Err(format!("Git commit failed: {}", error).into());
                }
            }
        }
        Err(error) => {
            return Err(format!("Git add failed: {}", error).into());
        }
    }

    // Step 4: Push changes to remote
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

fn main() {
    // let repo_path = "/home/rahul/Desktop/Node_code/git_wrapper";
    let repo_path = "/home/rahul/Desktop/Rust_code/git_wrapper_rust";
    let remote = "origin";
    let branch = "main";

    let op = git_push(repo_path, &remote, &branch);
    // let op = git_pull(repo_path, &remote, &branch);
    // let op = git_status(repo_path);
    println!("{:?}", op);
}
