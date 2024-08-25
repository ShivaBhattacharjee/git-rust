use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{self};
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};

pub struct Commit {
    pub id: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub parent: Option<String>,
}

pub struct Branch {
    pub name: String,
    pub head: String,
}

pub struct Repository {
    path: PathBuf,
    pub current_branch: String,
    pub branches: HashMap<String, Branch>,
    pub commits: HashMap<String, Commit>,
    pub staging_area: HashSet<PathBuf>,
}

impl Repository {
    pub fn new(path: &Path) -> io::Result<Self> {
        let repo_path = path.join(".rust-git");
        fs::create_dir_all(&repo_path)?;
        fs::create_dir_all(repo_path.join("objects"))?;
        fs::create_dir_all(repo_path.join("refs/heads"))?;

        let mut repo = Repository {
            path: repo_path,
            current_branch: "master".to_string(),
            branches: HashMap::new(),
            commits: HashMap::new(),
            staging_area: HashSet::new(),
        };

        repo.branches.insert("master".to_string(), Branch {
            name: "master".to_string(),
            head: "".to_string(),
        });

        Ok(repo)
    }

    pub fn create_file_or_folder(&self, path: &Path) -> io::Result<()> {
        if path.extension().is_some() {
            // It's a file
            File::create(path)?;
            println!("Created file: {}", path.display());
        } else {
            // It's a folder
            fs::create_dir_all(path)?;
            println!("Created folder: {}", path.display());
        }
        Ok(())
    }
    
    pub fn hash_object(&self, data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    pub fn write_object(&self, data: &[u8]) -> io::Result<String> {
        let hash = self.hash_object(data);
        let object_path = self.path.join("objects").join(&hash[..2]).join(&hash[2..]);
        fs::create_dir_all(object_path.parent().unwrap())?;
        fs::write(object_path, data)?;
        Ok(hash)
    }

    pub fn read_object(&self, hash: &str) -> io::Result<Vec<u8>> {
        let object_path = self.path.join("objects").join(&hash[..2]).join(&hash[2..]);
        fs::read(object_path)
    }

    pub fn commit(&mut self, message: &str) -> io::Result<String> {
        let timestamp = Utc::now();
        let parent = self.branches.get(&self.current_branch).unwrap().head.clone();
        
        let commit_data = format!(
            "tree {}\nparent {}\nmessage {}\ntimestamp {}",
            self.write_tree()?,
            parent,
            message,
            timestamp.to_rfc3339()
        );

        let commit_hash = self.write_object(commit_data.as_bytes())?;
        
        let commit = Commit {
            id: commit_hash.clone(),
            message: message.to_string(),
            timestamp,
            parent: if parent.is_empty() { None } else { Some(parent) },
        };

        self.commits.insert(commit_hash.clone(), commit);
        if let Some(branch) = self.branches.get_mut(&self.current_branch) {
            branch.head = commit_hash.clone();
        } else {
            println!("Failed to find the current branch to update head");
        }
        println!("Commit inserted: {}", commit_hash); // Debugging line
        self.branches.get_mut(&self.current_branch).unwrap().head = commit_hash.clone();
        println!("Head updated to: {}", commit_hash); // Debugging line
        self.staging_area.clear();

        Ok(commit_hash)
    }

    pub fn write_tree(&self) -> io::Result<String> {
        let mut tree_content = String::new();
        for path in &self.staging_area {
            if path.is_file() {
                let content = fs::read(&path)?;
                let hash = self.write_object(&content)?;
                tree_content.push_str(&format!("blob {} {}\n", hash, path.file_name().unwrap().to_str().unwrap()));
            }
        }
        self.write_object(tree_content.as_bytes())
    }
    pub fn log(&self) -> Vec<&Commit> {
        let mut commits = Vec::new();
        let mut current = self.branches.get(&self.current_branch).unwrap().head.clone();
        if current.is_empty() {
            println!("No commits found for the current branch");
            return commits;
        }
        println!("Starting log traversal from head: {}", current);
        while let Some(commit) = self.commits.get(&current) {
            commits.push(commit);
            println!("Traversing commit: {}", commit.id);
            if let Some(parent) = &commit.parent {
                current = parent.clone();
            } else {
                break;
            }
        }
        commits
    }

    pub fn branch(&mut self, name: &str) -> io::Result<()> {
        let current_head = self.branches.get(&self.current_branch).unwrap().head.clone();
        self.branches.insert(name.to_string(), Branch {
            name: name.to_string(),
            head: current_head,
        });
        Ok(())
    }

    pub fn checkout(&mut self, branch: &str) -> io::Result<()> {
        if !self.branches.contains_key(branch) {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Branch not found"));
        }
        self.current_branch = branch.to_string();
        Ok(())
    }

    pub fn diff(&self, file_path: &Path) -> io::Result<String> {
        let current_content = fs::read_to_string(file_path)?;
        let head_commit = self.branches.get(&self.current_branch).unwrap().head.clone();
        let head_tree = self.read_object(&head_commit)?;
        let head_tree_content = String::from_utf8_lossy(&head_tree);
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        
        for line in head_tree_content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts[2] == file_name {
                let old_content = self.read_object(parts[1])?;
                let old_content = String::from_utf8_lossy(&old_content);
                return Ok(diff_strings(&old_content, &current_content));
            }
        }
        
        Ok("File not found in the previous commit".to_string())
    }

    pub fn add(&mut self, path: &Path) -> io::Result<()> {
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let entry_path = entry.path();
                if entry_path.is_file() {
                    self.staging_area.insert(entry_path);
                } else if entry_path.is_dir() {
                    self.add(&entry_path)?;
                }
            }
        } else if path.is_file() {
            self.staging_area.insert(path.to_path_buf());
        }
        Ok(())
    }
}

pub fn diff_strings(old: &str, new: &str) -> String {
    let mut result = String::new();
    let old_lines: Vec<&str> = old.lines().collect();
    let new_lines: Vec<&str> = new.lines().collect();

    for (i, (old_line, new_line)) in old_lines.iter().zip(new_lines.iter()).enumerate() {
        if old_line != new_line {
            result.push_str(&format!("Line {}: \n- {}\n+ {}\n", i + 1, old_line, new_line));
        }
    }

    if old_lines.len() < new_lines.len() {
        for (i, line) in new_lines.iter().enumerate().skip(old_lines.len()) {
            result.push_str(&format!("Line {}: \n+ {}\n", i + 1, line));
        }
    } else if old_lines.len() > new_lines.len() {
        for (i, line) in old_lines.iter().enumerate().skip(new_lines.len()) {
            result.push_str(&format!("Line {}: \n- {}\n", i + 1, line));
        }
    }

    result
}