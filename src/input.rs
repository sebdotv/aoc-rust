use std::path::Path;

use anyhow::{Context, Result};

pub fn read_data_file(
    parent_dir_name: &str,
    dir_name: &str,
    data_file_name: &str,
) -> Result<String> {
    let data_file_path = Path::new("data")
        .join(parent_dir_name)
        .join(dir_name)
        .join(data_file_name);
    let data = std::fs::read_to_string(data_file_path.clone())
        .with_context(|| format!("Could not open file {:?}", data_file_path))?;
    Ok(data)
}
