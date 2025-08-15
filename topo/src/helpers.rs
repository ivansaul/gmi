use crate::error::Result;
use std::{fs, path::PathBuf};

pub fn list_csv_files() -> Result<Vec<PathBuf>> {
    let mut csv_files = Vec::new();
    let current_dir = std::env::current_dir()?;

    for entry in fs::read_dir(&current_dir)? {
        let entry = entry?;
        let path: PathBuf = entry.path();

        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if ext.eq_ignore_ascii_case("csv") {
                csv_files.push(path);
            }
        }
    }
    Ok(csv_files)
}
