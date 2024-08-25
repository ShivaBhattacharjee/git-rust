use rust_git::Repository;
use std::env;
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: rust-git-cli <command> [<args>]");
        return Ok(());
    }

    let repo_path = Path::new(".rust-cli");
    if !repo_path.exists() {
        fs::create_dir(repo_path)?;
    }

    let mut repo = Repository::new(repo_path)?;
    
    let commit_hash = repo.commit(&args[1])?;
    println!("Created commit: {}", commit_hash);

    Ok(())
}