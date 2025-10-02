// Library interface for treecraft
// Exposes public API for building directory structures

mod tree;
mod parser;
mod creator;
mod error;

pub use error::TreeCraftError;
pub use tree::Node;
pub use parser::{Parser, BoxDrawingParser};

type Result<T> = std::result::Result<T, TreeCraftError>;

/// Build a directory structure from input file
pub fn build_structure(
    input_path: &str,
    output_path: &str,
    preview: bool,
) -> Result<()> {
    // Read input file
    let contents = std::fs::read_to_string(input_path)
        .map_err(TreeCraftError::FileSystem)?;
    
    // Parse using default parser
    let parser = BoxDrawingParser;
    let root = parser.parse(&contents)?;
    
    if preview {
        creator::preview(&root, output_path);
        Ok(())
    } else {
        creator::create(&root, output_path)
    }
}