mod grabber;
use grabber::{python, rust};

use copypasta::{ClipboardContext, ClipboardProvider};
use std::io::{self, Result};
use std::env;

fn main() -> Result<()> {
    let current_dir = env::current_dir()?;

    // 1. Check if this is a Rust project.
    if let Some(rust_dir) = rust::find_rust_project(current_dir.clone()) {
        println!("Detected a Rust project at: {}\n", rust_dir.display());
        let aggregated_contents = rust::grab_rust(&rust_dir)?;
        if aggregated_contents.is_empty() {
            println!("No `.rs` files found or no content was aggregated.");
            return Ok(());
        }
        copy_to_clipboard(aggregated_contents);
        return Ok(());
    }

    // 2. If not Rust, check if it’s a Python project.
    if let Some(py_dir) = python::find_python_project(current_dir.clone()) {
        println!("Detected a Python project at: {}\n", py_dir.display());
        let aggregated_contents = python::grab_python(&py_dir)?;
        if aggregated_contents.is_empty() {
            println!("No `.py` files found or no content was aggregated.");
            return Ok(());
        }
        copy_to_clipboard(aggregated_contents);
        return Ok(());
    }

    // 3. Otherwise, not Rust or Python
    eprintln!("No Rust or Python project detected in this or up to two parent directories.");
    Ok(())
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
