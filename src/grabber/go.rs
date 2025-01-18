use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use super::tree::FileNode;

/// Returns `Some(path)` if we find a Go project by searching up to two levels.
/// A “Go project” is defined by having a `go.mod` file or a `src` directory.
pub fn find_go_project(mut start_dir: PathBuf) -> Option<PathBuf> {
    for _ in 0..3 {
        if start_dir.join("go.mod").exists() || start_dir.join("src").is_dir() {
            return Some(start_dir.clone());
        }
        if !start_dir.pop() {
            break;
        }
    }
    None
}

/// Aggregates `.go` files from the given `go_dir`.
/// Also prints a tree only of the `.go` files.
pub fn grab_go(go_dir: &Path) -> io::Result<String> {
    // 1. Collect `.go` files
    let mut go_files = Vec::new();
    for entry in WalkDir::new(go_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "go" {
                go_files.push(path.to_path_buf());
            }
        }
    }

    if go_files.is_empty() {
        return Ok(String::new());
    }

    // 2. Build a tree of `.go` files
    let mut root_node = FileNode::new("");
    for file_path in &go_files {
        let relative_path = file_path.strip_prefix(go_dir).unwrap_or(file_path);
        let components: Vec<String> = relative_path
            .components()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .collect();
        root_node.insert(&components);
    }

    println!("Tree of `.go` files in this Go project:");
    root_node.print(0);
    println!();

    // 3. Aggregate
    let mut aggregated_contents = String::new();
    for file_path in go_files {
        let mut file = File::open(&file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        aggregated_contents.push_str(&format!("=== {} ===\n", file_path.display()));
        aggregated_contents.push_str(&contents);
        aggregated_contents.push_str("\n\n");
    }

    Ok(aggregated_contents)
}
