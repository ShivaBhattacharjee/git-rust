
use rust_git::Repository;
use std::env;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: diff-service <file>");
        return Ok(());
    }

    let repo_path = Path::new(".");
    let repo = Repository::new(repo_path)?;
    
    let diff = repo.diff(Path::new(&args[1]))?;
    println!("{}", diff);

    Ok(())
}