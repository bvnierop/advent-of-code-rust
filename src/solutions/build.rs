use std::fs;
use std::path::{Path, PathBuf};
use heck::ToSnakeCase;

fn main() {
    // Define the directory where solution files are located
    let solutions_dir = Path::new("src");

    // Collect module declarations
    let mut modules = String::new();
    visit_dirs(&solutions_dir, &mut modules, 0).unwrap();

    // Write the modules to all_modules.rs in src/solutions/src
    let dest_path = PathBuf::from("src").join("lib.rs");
    fs::write(&dest_path, modules).unwrap();

    // Instruct Cargo to rerun the build script when files in src change
    println!("cargo:rerun-if-changed=src");
}

fn visit_dirs(dir: &Path, modules: &mut String, depth: usize) -> std::io::Result<()> {
    if dir.is_dir() {
        if depth != 0 {
            if let Some(folder_name) = dir.file_name().and_then(|n| n.to_str()) {
                let indent = "    ".repeat(depth - 1);
                modules.push_str(&format!("{}#[path = \"{}\"]\n", indent, folder_name));
                modules.push_str(&format!("{}pub mod y{} {{\n", indent, folder_name));
            }
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_dirs(&path, modules, depth + 1)?;
            } else if let Some(ext) = path.extension() {
                if ext == "rs" {
                    if let Some(file_stem) = path.file_stem().and_then(|n| n.to_str()) {
                        if file_stem != "mod" && file_stem != "lib" {
                            let indent = "    ".repeat(depth);

                            let file_name = path.file_name().unwrap();

                            modules.push_str(&format!(
                                "{}#[path = \"{}\"]\n",
                                indent,
                                file_name.to_str().unwrap()
                            ));

                            modules.push_str(&format!("{}pub mod d{};\n", indent, file_stem.to_snake_case()));
                        }
                    }
                }
            }
        }

        if depth != 0 {
            let indent = "    ".repeat(depth - 1);
            modules.push_str(&format!("{}}}\n", indent));
        }
    }
    Ok(())
}
