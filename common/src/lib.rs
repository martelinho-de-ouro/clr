use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// dyn keyword here says that the return type's trait is dynamically
/// dispatched allowing to abstract the idea of input source in case
/// the source implements the trait BufRead.
/// The return needs to be boxed because the compiler has no idea
/// about the size of of the thing that will be read at compile time.
pub fn open(file: &str) -> Result<Box<dyn BufRead>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?))),
    }
}
