use rust_git::Repository;
use std::env;
use std::io;
use std::path::Path;
use std::fs;
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let repo_path = Path::new(".");
    let mut repo = Repository::new(repo_path)?;

    match args[1].as_str() {
        "init" => {
            println!("Initialized empty Rust-Git repository");
        }
        "branch" => {
            if args.len() == 2 {
                // List all branches
                for (name, _branch) in &repo.branches {
                    let marker = if name == &repo.current_branch {
                        "*"
                    } else {
                        " "
                    };
                    println!("{} {}", marker, name);
                }
            } else {
                // Create new branch (existing implementation)
                repo.branch(&args[2])?;
                println!("Created branch: {}", args[2]);
            }
        }
        "checkout" => {
            if args.len() < 3 {
                println!("Usage: rust-git checkout <branch>");
                return Ok(());
            }

            let branch_name = args[2].clone();
            if branch_name == "-b" {
                if args.len() < 4 {
                    println!("Usage: rust-git checkout -b <branch>");
                    return Ok(());
                }
                let new_branch_name = &args[3];
                repo.branch(new_branch_name)?;
                repo.checkout(new_branch_name)?;
                println!("Switched to new branch: {}", new_branch_name);
            } else {
                match repo.checkout(&branch_name) {
                    Ok(_) => println!("Switched to branch: {}", branch_name),
                    Err(_) => println!("Branch not found: {}", branch_name),
                }
            }
        }
        "show" => {
            if args.len() < 3 {
                println!("Usage: rust-git show <commit-id>");
                return Ok(());
            }
            if let Some(commit) = repo.commits.get(&args[2]) {
                println!("Commit: {}", commit.id);
                println!("Message: {}", commit.message);
                println!("Timestamp: {}", commit.timestamp);
                println!("Parent: {:?}", commit.parent);
            } else {
                println!("Commit not found");
            }
        }
        "commit" => {
            if args.len() < 3 {
                println!("Usage: rust-git commit <message>");
                return Ok(());
            }
            let commit_hash = repo.commit(&args[2])?;
            println!("Created commit: {}", commit_hash);
        }
        "log" => {
            println!("Logs are");
            for commit in repo.log() {
                println!("Commit: {}", commit.id);
                println!("Message: {}", commit.message);
                println!("Timestamp: {}", commit.timestamp);
                println!();
            }
        }
        "status" => {
            println!("Changes not staged for commit:");
            for entry in fs::read_dir(".")? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && path.file_name().unwrap() != ".rust-git" {
                    println!("  modified: {}", path.display());
                }
            }
        }
        "diff" => {
            if args.len() < 3 {
                println!("Usage: rust-git diff <file>");
                return Ok(());
            }
            let diff = repo.diff(Path::new(&args[2]))?;
            println!("{}", diff);
        }
        "create" => {
            if args.len() < 3 {
                println!("Usage: rust-git create <file_or_folder_path>");
                return Ok(());
            }
            repo.create_file_or_folder(Path::new(&args[2]))?;
        }
        "add" => { 
            if args.len() < 3 {
                println!("Usage: rust-git add <file_or_folder_path>");
                return Ok(());
            }
            repo.add(Path::new(&args[2]))?;
            println!("Added {} to staging area", args[2]);
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            print_usage();
        }
    }

    Ok(())
}
fn print_usage() {
    println!("Usage: rust-git <command> [<args>]");
    println!("Available commands:");
    println!("  init");
    println!("  commit <message>");
    println!("  log");
    println!("  branch <name>");
    println!("  checkout <branch>");
    println!("  diff <file>");
    println!("  create <file_or_folder_path>");
}
