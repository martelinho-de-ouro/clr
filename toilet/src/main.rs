use std::{fs::File, io::BufReader};

use anyhow::Result;
use rodio::{source::Source, Decoder, OutputStream};

enum Intensity {
    Small(String),
    Medium(String),
    Big(String),
}

fn flush(intensity: Intensity) -> Result<(), anyhow::Error> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    match intensity {
        Intensity::Small(i) => {
            let file = BufReader::new(File::open(i)?);
            let source = Decoder::new(file)?;
            stream_handle.play_raw(source.convert_samples())?;
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
        Intensity::Medium(i) => {
            let file = BufReader::new(File::open(i)?);
            let source = Decoder::new(file)?;
            stream_handle.play_raw(source.convert_samples())?;
            std::thread::sleep(std::time::Duration::from_secs(4));
        }
        Intensity::Big(i) => {
            let file = BufReader::new(File::open(i)?);
            let source = Decoder::new(file)?;
            stream_handle.play_raw(source.convert_samples())?;
            std::thread::sleep(std::time::Duration::from_secs(9));
        }
    }
    Ok(())
}

fn main() {
    let small = Intensity::Small(String::from("toilet/resources/small.mp3"));
    let medium = Intensity::Medium(String::from("toilet/resources/medium.mp3"));
    let big = Intensity::Big(String::from("toilet/resources/big.mp3"));

    let _ = flush(small);
    let _ = flush(medium);
    let _ = flush(big);
}
