
use std::fs::{File, read_dir};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use super::tree::FileNode;

/// Returns true if the given directory contains at least one `.py` file
/// in its **top-level contents** (not checking subdirectories).
fn has_python_file_in_top_level(dir: &Path) -> bool {
    let entries = match read_dir(dir) {
        Ok(dir_iter) => dir_iter,
        Err(_) => return false, 
    };

    for entry_result in entries {
        if let Ok(entry) = entry_result {
            let path = entry.path();
            // We only care about `.py` files in the top level
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "py" {
                        return true;
                    }
                }
            }
        }
    }

    false
}

/// Returns `Some(path)` if we find a “Python project” 
/// by searching **up to two levels** above `start_dir`
/// where:
/// - The directory contains a `requirements.txt` or `setup.py` **OR**
/// - The directory itself contains at least one `.py` file (in top-level).
pub fn find_python_project(mut start_dir: PathBuf) -> Option<PathBuf> {
    for _ in 0..3 {
        let has_req = start_dir.join("requirements.txt").exists();
        let has_setup = start_dir.join("setup.py").exists();
        let has_top_level_py = has_python_file_in_top_level(&start_dir);

        if has_req || has_setup || has_top_level_py {
            return Some(start_dir);
        }
        if !start_dir.pop() {
            break;
        }
    }
    None
}

/// Finds and aggregates all `.py` files (recursively) in `py_dir`,
/// prints a tree of them (relative to `py_dir`),
/// and returns the aggregated contents.
pub fn grab_python(py_dir: &Path) -> io::Result<String> {
    // 1. Gather `.py` files
    let mut py_files = Vec::new();
    for entry in WalkDir::new(py_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "py" {
                py_files.push(path.to_path_buf());
            }
        }
    }

    if py_files.is_empty() {
        return Ok(String::new());
    }

    // 2. Build a tree of `.py` files
    let mut root_node = FileNode::new("");
    for file_path in &py_files {
        let relative_path = file_path.strip_prefix(py_dir).unwrap_or(file_path);
        let components: Vec<String> = relative_path
            .components()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .collect();

        root_node.insert(&components);
    }

    println!("Tree of `.py` files in this Python project:");
    root_node.print(0);
    println!();

    // 3. Aggregate
    let mut aggregated_contents = String::new();
    for file_path in py_files {
        let mut file = File::open(&file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        aggregated_contents.push_str(&format!("=== {} ===\n", file_path.display()));
        aggregated_contents.push_str(&contents);
        aggregated_contents.push_str("\n\n");
    }

    Ok(aggregated_contents)
}
