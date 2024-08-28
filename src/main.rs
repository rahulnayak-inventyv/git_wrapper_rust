use std::{path::Path, process::Command};

fn git_status(repo_path: &str) -> Result<std::process::Output, std::io::Error> {
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
fn git_pull(repo_path: &str, remote: &str, branch: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo_path = Path::new(repo_path);

    if !repo_path.exists() || !repo_path.is_dir() {
        return Err(format!("Invalid repository path: {}", repo_path.display()).into());
    }

    let mut binding = Command::new("git");
    let command = binding.current_dir(repo_path);
    let pull_op = command.args(&["pull", "-v", remote, branch]).output();
    match pull_op {
        Ok(op) => {
            println!("op===>{:?}", op);
        }
        Err(error) => {
            eprintln!("{}", error);
        }
    }
    Ok(())
}
fn git_push(repo_path: &str, remote: &str, branch: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo_path = Path::new(repo_path);

    if !repo_path.exists() || !repo_path.is_dir() {
        return Err(format!("Invalid repository path: {}", repo_path.display()).into());
    }
    let status_output = git_status(repo_path.to_str().unwrap());
    match status_output {
        Ok(status_output) => {
            if status_output.stdout.is_empty() {
                println!("No changes to push.");
                return Ok(());
            }
        }
        Err(error) => {
            return Err(format!("{}", error).into());
        }
    }
    println!("Changes to be pushed:");
    let diff_output = Command::new("git")
        .current_dir(repo_path)
        .args(&["add", "./*"])
        .output();
    match diff_output {
        Ok(op) => {
            println!("Git OP::>{:?}", op);
            let commit = Command::new("git")
                .current_dir(repo_path)
                .args(&["commit", "-m", "committed by code"])
                .output();
            match commit {
                Ok(commit_op) => {
                    println!("Git commit :{:?}", commit_op);
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
                    eprint!("{}", error);
                    Err(format!("{}", error).into())
                }
            }
        }
        Err(error) => {
            println!("37:::>{}", error);
            Err(format!("37:::>{}", error).into())
        }
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
