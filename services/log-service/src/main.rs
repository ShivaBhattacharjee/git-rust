use rust_git::Repository;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let repo_path = Path::new(".");
    let repo = Repository::new(repo_path)?;
    
    for commit in repo.log() {
        println!("Commit: {}", commit.id);
        println!("Message: {}", commit.message);
        println!("Timestamp: {}", commit.timestamp);
        println!();
    }

    Ok(())
}