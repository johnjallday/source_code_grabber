use std::collections::BTreeMap;

/// A simple node in a tree of file paths.
#[derive(Debug)]
pub struct FileNode {
    pub name: String,
    pub children: BTreeMap<String, FileNode>,
}

impl FileNode {
    /// Create a new `FileNode` with the given `name`.
    pub fn new(name: &str) -> Self {
        FileNode {
            name: name.to_owned(),
            children: BTreeMap::new(),
        }
    }

    /// Insert a path slice (list of path components) into the tree.
    pub fn insert(&mut self, path_components: &[String]) {
        if path_components.is_empty() {
            return;
        }

        let child_name = &path_components[0];
        let child_node = self
            .children
            .entry(child_name.clone())
            .or_insert_with(|| FileNode::new(child_name));
        child_node.insert(&path_components[1..]);
    }

    /// Print this node and its children in a tree structure.
    /// `level` controls indentation depth.
    pub fn print(&self, level: usize) {
        if level > 0 {
            let indent = "  ".repeat(level - 1);
            println!("{}{}", indent, self.name);
        }
        for child in self.children.values() {
            child.print(level + 1);
        }
    }
}
