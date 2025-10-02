// CLI interface

use tree_craft::{build_structure, TreeCraftError};

fn main() -> Result<(), TreeCraftError> {
    let args: Vec<String> = std::env::args().collect();
    
    // Check for preview flag
    let preview_mode = args.contains(&"--preview".to_string());
    
    // Filter out flags to get positional arguments
    let non_flag_args: Vec<&String> = args.iter().filter(|a| !a.starts_with("--")).collect();
    
    if non_flag_args.len() != 3 {  // program name + 2 args
        eprintln!("Usage: {} [--preview] <structure.txt> <output-directory>", args[0]);
        eprintln!("\nOptions:");
        eprintln!("  --preview    Show what would be created without creating files");
        std::process::exit(1);
    }
    
    let input_file = non_flag_args[1];
    let output_dir = non_flag_args[2];
    
    if preview_mode {
        println!("Preview of structure to be created:\n");
    } else {
        println!("Creating structure at: {}", output_dir);
    }
    
    build_structure(input_file, output_dir, preview_mode)?;
    
    if !preview_mode {
        println!("✓ Successfully created structure");
    }
    
    Ok(())
}