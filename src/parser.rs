// Parser trait and implementations for different text formats

use crate::tree::Node;
use crate::error::TreeCraftError;

type Result<T> = std::result::Result<T, TreeCraftError>;

/// Trait for parsing text into tree structures
pub trait Parser {
    fn parse(&self, contents: &str) -> Result<Node>;
}

/// Parser for box-drawing style trees (├──, └──, │)
pub struct BoxDrawingParser;

impl Parser for BoxDrawingParser {
    fn parse(&self, contents: &str) -> Result<Node> {
        let lines: Vec<&str> = contents.lines().collect();
        
        if lines.is_empty() {
            return Err(TreeCraftError::Parse("Empty file".to_string()));
        }
        
        let root_name = extract_name(lines[0]);
        let mut root = Node::new(root_name, true);
        
        // Collect (indent_level, name, is_directory) for each line
        let mut items: Vec<(usize, String, bool)> = vec![];
        
        for line in lines.iter().skip(1) {
            if line.trim().is_empty() {
                continue;
            }
            
            let indent = get_indent_level(line);
            let name = extract_name(line);
            
            if name.is_empty() {
                continue;
            }
            
            let is_dir = is_directory(&name);
            let clean_name = name.trim_end_matches('/').to_string();
            
            items.push((indent, clean_name, is_dir));
        }
        
        build_tree(&mut root, &items, 0, 0);
        
        Ok(root)
    }
}

// Calculate indentation level from tree notation
fn get_indent_level(line: &str) -> usize {
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    let mut level = 0;
    
    // Count leading whitespace
    let mut leading_whitespace = 0;
    while i < chars.len() && (chars[i] == ' ' || chars[i] == '\t') {
        leading_whitespace += if chars[i] == ' ' { 1 } else { 4 };
        i += 1;
    }
    
    if i < chars.len() && chars[i] == '│' {
        // Box-drawing: │ = passing through parent level
        i += 1;
        level += 1;
        
        let space_start = i;
        while i < chars.len() && (chars[i] == ' ' || chars[i] == '\t') {
            i += 1;
        }
        let spaces_after_pipe = i - space_start;
        
        // Base spacing is 3, each +4 spaces = another level
        if spaces_after_pipe >= 3 {
            level += 1;
            if spaces_after_pipe > 3 {
                level += (spaces_after_pipe - 3) / 4;
            }
        }
    } else if i < chars.len() && (chars[i] == '├' || chars[i] == '└') {
        // Plain indentation: each 4 spaces = one level
        level = (leading_whitespace / 4) + 1;
    }
    
    level
}

// Extract name, removing tree characters and comments
fn extract_name(line: &str) -> String {
    let name = line
        .replace('│', "")
        .replace('├', "")
        .replace('└', "")
        .replace('─', "")
        .trim()
        .to_string();
    
    // Strip comments after #
    if let Some(pos) = name.find('#') {
        name[..pos].trim().to_string()
    } else {
        name
    }
}

// Directory if ends with / or has no extension
fn is_directory(name: &str) -> bool {
    if name.ends_with('/') {
        return true;
    }
    
    let last_part = name.split('/').last().unwrap_or(name);
    !last_part.contains('.')
}

// Build tree recursively from flat list
fn build_tree(parent: &mut Node, items: &[(usize, String, bool)], start_idx: usize, parent_level: usize) -> usize {
    let mut i = start_idx;
    
    while i < items.len() {
        let (indent, name, is_dir) = &items[i];
        
        if *indent == parent_level + 1 {
            // Direct child
            let mut child = Node::new(name.clone(), *is_dir);
            
            if *is_dir {
                i = build_tree(&mut child, items, i + 1, *indent);
            } else {
                i += 1;
            }
            
            parent.add_child(child);
        } else if *indent <= parent_level {
            return i;  // Back to parent level
        } else {
            return i;  // Skip deeper items
        }
    }
    
    i
}