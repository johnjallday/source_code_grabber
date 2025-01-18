mod grabber;

use grabber::{python, rust, go}; // <-- Add `go` here

use copypasta::{ClipboardContext, ClipboardProvider};
use std::env;
use std::io::{self, Result};
use std::path::PathBuf;

fn main() -> Result<()> {
    // 0. Find project_info.toml
    let current_dir = env::current_dir()?;
    let Some(project_info_dir) = find_project_info_toml(current_dir.clone()) else {
        eprintln!("No `project_info.toml` found in current or up to two parent directories.");
        return Ok(());
    };

    println!("Found `project_info.toml` in: {}\n", project_info_dir.display());

    // 1. Check if this is a Rust project, starting from that directory.
    if let Some(rust_dir) = rust::find_rust_project(project_info_dir.clone()) {
        println!("Detected a Rust project at: {}\n", rust_dir.display());
        let aggregated_contents = rust::grab_rust(&rust_dir)?;
        if aggregated_contents.is_empty() {
            println!("No `.rs` files found or no content was aggregated.");
            return Ok(());
        }
        copy_to_clipboard(aggregated_contents);
        return Ok(());
    }

    // 2. If not Rust, check if it’s a Python project, starting from that directory.
    if let Some(py_dir) = python::find_python_project(project_info_dir.clone()) {
        println!("Detected a Python project at: {}\n", py_dir.display());
        let aggregated_contents = python::grab_python(&py_dir)?;
        if aggregated_contents.is_empty() {
            println!("No `.py` files found or no content was aggregated.");
            return Ok(());
        }
        copy_to_clipboard(aggregated_contents);
        return Ok(());
    }

    // 3. If not Rust or Python, check if it’s a Go project.
    if let Some(go_dir) = go::find_go_project(project_info_dir.clone()) {
        println!("Detected a Go project at: {}\n", go_dir.display());
        let aggregated_contents = go::grab_go(&go_dir)?;
        if aggregated_contents.is_empty() {
            println!("No `.go` files found or no content was aggregated.");
            return Ok(());
        }
        copy_to_clipboard(aggregated_contents);
        return Ok(());
    }

    // 4. Otherwise, not Rust, Python, or Go
    eprintln!("`project_info.toml` found, but no Rust, Python, or Go project detected within.");
    Ok(())
}

/// Search current directory (and up to two parents) for `project_info.toml`.
fn find_project_info_toml(mut start_dir: PathBuf) -> Option<PathBuf> {
    for _ in 0..3 {
        if start_dir.join("project_info.toml").exists() {
            return Some(start_dir);
        }
        if !start_dir.pop() {
            break;
        }
    }
    None
}

fn copy_to_clipboard(contents: String) {
    match ClipboardContext::new() {
        Ok(mut ctx) => {
            match ctx.set_contents(contents) {
                Ok(_) => println!("Aggregated contents copied to clipboard successfully!"),
                Err(e) => eprintln!("Failed to copy to clipboard: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to initialize clipboard context: {}", e),
    }
}
