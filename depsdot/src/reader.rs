use std::fs::File;
use std::io::Read;
use std::io;
use std::path::PathBuf;

pub fn read_content(path: PathBuf) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}
