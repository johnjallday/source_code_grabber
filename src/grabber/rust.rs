use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

// Import the FileNode from our common `tree` module
use super::tree::FileNode;

pub fn find_rust_project(mut start_dir: PathBuf) -> Option<PathBuf> {
    // We allow checking current + up to 2 parents.
    for _ in 0..3 {
        if start_dir.join("Cargo.toml").exists() && start_dir.join("src").is_dir() {
            return Some(start_dir);
        }
        if !start_dir.pop() {
            break;
        }
    }
    None
}

/// Aggregates `.rs` files from the given `rust_dir`.
/// Also prints a tree only of the `.rs` files.
pub fn grab_rust(rust_dir: &Path) -> io::Result<String> {
    // 1. Collect `.rs` files
    let mut rs_files = Vec::new();
    for entry in WalkDir::new(rust_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "rs" {
                rs_files.push(path.to_path_buf());
            }
        }
    }

    if rs_files.is_empty() {
        return Ok(String::new());
    }

    // 2. Build a tree of `.rs` files
    let mut root_node = FileNode::new("");
    for file_path in &rs_files {
        let relative_path = file_path.strip_prefix(rust_dir).unwrap_or(file_path);
        let components: Vec<String> = relative_path
            .components()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .collect();

        root_node.insert(&components);
    }

    println!("Tree of `.rs` files in this Rust project:");
    root_node.print(0);
    println!();

    // 3. Aggregate
    let mut aggregated_contents = String::new();
    for file_path in rs_files {
        let mut file = File::open(&file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        aggregated_contents.push_str(&format!("=== {} ===\n", file_path.display()));
        aggregated_contents.push_str(&contents);
        aggregated_contents.push_str("\n\n");
    }

    Ok(aggregated_contents)
}
