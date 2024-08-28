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
            println!("=============>{:?}",commit);
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
    // let diff_output = Command::new("git")
    //     .arg("-C")
    //     .arg(repo_path)
    //     .arg("status")
    //     .arg("--porcelain")
    //     .output();
    // match diff_output {
    //     Ok(diff_output) => {
    //         if diff_output.stdout.is_empty() {
    //             println!("No changes to push.");
    //         } else {
    //             println!("Changes detected, pushing to the remote repository...");

    //             // Git Push
    //             let push_output = Command::new("git")
    //                 .arg("-C")
    //                 .arg(repo_path)
    //                 .arg("push")
    //                 .output();
    //             match push_output {
    //                 Ok(push_output) => {
    //                     if push_output.status.success() {
    //                         println!("Git push was successful!");
    //                         let res = io::stdout().write_all(&push_output.stdout);
    //                         match res {
    //                             Ok(_) => {
    //                                 println!("Git push was successful!");
    //                             }
    //                             Err(error) => {
    //                                 println!("success:::{}", error);
    //                             }
    //                         }
    //                     } else {
    //                         eprintln!("Git push failed.");
    //                         let res = io::stderr().write_all(&push_output.stderr);
    //                         match res {
    //                             Ok(_) => {
    //                                 eprintln!("Git push failed.");
    //                             }
    //                             Err(error) => {
    //                                 println!("error::{}", error);
    //                             }
    //                         }
    //                     }
    //                 }
    //                 Err(error) => {
    //                     println!("54:::>{}", error);
    //                 }
    //             }
    //         }
    //     }
    //     Err(error) => {
    //         println!("38::>{}", error);
    //     }
    // }

    // Run the 'git status' command
    // let output = Command::new("git")
    //     .arg("-C")
    //     .arg(repo_path)
    //     .arg("push")
    //     .output();
    // match output {
    //     Ok(output) => {
    //         if output.status.success() {
    //             let res = io::stdout().write_all(&output.stdout);
    //             match res {
    //                 Ok(_) => {
    //                     println!("Git push was successful!");
    //                 }
    //                 Err(error) => {
    //                     println!("success:::{}", error);
    //                 }
    //             }
    //         } else {
    //             let res = io::stderr().write_all(&output.stderr);
    //             match res {
    //                 Ok(_) => {
    //                     eprintln!("Git push failed.");
    //                 }
    //                 Err(error) => {
    //                     println!("error::{}", error);
    //                 }
    //             }
    //         }
    //     }
    //     Err(error) => {
    //         println!("{}", error);
    //     }
    // }
}
