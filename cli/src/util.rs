use std::fs;
use std::fs::File;
use std::io::LineWriter;
use std::io::Write;
use std::path::Path;

/// write file, for generating classes files
pub fn write_file<P: AsRef<Path> + Clone>(
    path: P,
    content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let path_str = path.clone().as_ref().to_string_lossy().to_string();
    let file = File::create(path)?;
    let mut writer = LineWriter::new(file);
    writer.write_all(content.as_bytes())?;
    println!(
        "write_file: [{}] {}",
        content.len(), path_str
    );
    Ok(())
}

