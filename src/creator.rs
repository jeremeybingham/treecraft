// Creates files and directories from tree structure

use std::fs;
use std::path::Path;
use crate::tree::Node;
use crate::error::TreeCraftError;

type Result<T> = std::result::Result<T, TreeCraftError>;

pub fn create(root: &Node, output_dir: &str) -> Result<()> {
    // Error if output directory already exists
    if Path::new(output_dir).exists() {
        return Err(TreeCraftError::AlreadyExists(output_dir.to_string()));
    }
    
    // Create root directory
    fs::create_dir(output_dir)?;
    
    // Create all children
    for child in &root.children {
        create_node(child, Path::new(output_dir))?;
    }
    
    Ok(())
}

// Recursively create node and children on disk
fn create_node(node: &Node, base_path: &Path) -> Result<()> {
    let node_path = base_path.join(&node.name);
    
    if node.is_directory {
        fs::create_dir(&node_path)?;
        for child in &node.children {
            create_node(child, &node_path)?;
        }
    } else {
        fs::File::create(&node_path)?;
    }
    
    Ok(())
}

// Preview what would be created without actually creating it
pub fn preview(root: &Node, output_dir: &str) {
    println!("{}/", output_dir);
    for child in &root.children {
        preview_node(child, "", true);
    }
}

// Recursively print tree structure
fn preview_node(node: &Node, prefix: &str, is_last: bool) {
    // Choose branch character based on position
    let branch = if is_last { "└── " } else { "├── " };
    
    // Print this node
    if node.is_directory {
        println!("{}{}{}/", prefix, branch, node.name);
    } else {
        println!("{}{}{}", prefix, branch, node.name);
    }
    
    // Recursively print children
    if !node.children.is_empty() {
        // Extension: │ if more siblings, space if last
        let extension = if is_last { "    " } else { "│   " };
        let child_prefix = format!("{}{}", prefix, extension);
        
        let child_count = node.children.len();
        for (i, child) in node.children.iter().enumerate() {
            let child_is_last = i == child_count - 1;
            preview_node(child, &child_prefix, child_is_last);
        }
    }
}