use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    // Get the directory where the build script is executed
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(&out_dir).join("all_modules.rs");

    // Define the directory where solution files are located
    let solutions_dir = Path::new("src");

    // Collect module declarations
    let mut modules = String::new();
    visit_dirs(&solutions_dir, &mut modules).unwrap();

    // Write the modules to the output file
    fs::write(&dest_path, modules).unwrap();

    // Instruct Cargo to rerun the build script when files change
    println!("cargo:rerun-if-changed=src");
}

fn visit_dirs(dir: &Path, modules: &mut String) -> std::io::Result<()> {
    if dir.is_dir() {
        // Check if the directory is a year folder (e.g., y2023)
        if let Some(folder_name) = dir.file_name().and_then(|n| n.to_str()) {
            if folder_name.starts_with('y') {
                modules.push_str(&format!("pub mod {} {{\n", folder_name));
            }
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Recurse into subdirectories
                visit_dirs(&path, modules)?;
            } else if let Some(ext) = path.extension() {
                if ext == "rs" {
                    if let Some(file_stem) = path.file_stem().and_then(|n| n.to_str()) {
                        // Skip mod.rs and lib.rs files
                        if file_stem != "mod" && file_stem != "lib" {
                            modules.push_str(&format!("    pub mod {};\n", file_stem));
                        }
                    }
                }
            }
        }

        // Close the module block if it was a year folder
        if let Some(folder_name) = dir.file_name().and_then(|n| n.to_str()) {
            if folder_name.starts_with('y') {
                modules.push_str("}\n");
            }
        }
    }
    Ok(())
}