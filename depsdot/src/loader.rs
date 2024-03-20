use std::{fs, path::PathBuf};

/// Gets all the Cargo.toml files from a directory recursively.
///
/// It will skip `target`, `.git`, `.github` and `tests` directories.
pub fn load_cargo_tomls(dir: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut cargo_tomls = Vec::new();
    let entries = fs::read_dir(dir)?;

    for e in entries {
        let entry = e?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(name) = path.file_name() {
                if name == "target"
                    || name == ".git"
                    || name == ".github"
                    || name == "tests"
                {
                    continue;
                }
            }

            let temp = load_cargo_tomls(&path.to_string_lossy())?;
            cargo_tomls.extend(temp);
        } else if let Some(file_name) = path.file_name() {
            if file_name == "Cargo.toml" {
                cargo_tomls.push(path);
            }
        }
    }

    Ok(cargo_tomls)
}
