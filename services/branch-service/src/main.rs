use rust_git::Repository;
use std::env;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: branch-service <create|checkout> <branch-name>");
        return Ok(());
    }

    let repo_path = Path::new(".");
    let mut repo = Repository::new(repo_path)?;

    match args[1].as_str() {
        "create" => {
            repo.branch(&args[2])?;
            println!("Created branch: {}", args[2]);
        }
        "checkout" => {
            repo.checkout(&args[2])?;
            println!("Switched to branch: {}", args[2]);
        }
        _ => {
            println!("Unknown command: {}", args[1]);
        }
    }

    Ok(())
}