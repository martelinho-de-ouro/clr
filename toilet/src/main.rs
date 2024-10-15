use std::{fs::File, io::BufReader};

use anyhow::Result;
use rodio::{source::Source, Decoder, OutputStream, OutputStreamHandle};

enum Intensity {
    Small(String),
    Medium(String),
    Big(String),
}

fn play_sound(
    path: &str,
    duration: u64,
    stream_handle: &OutputStreamHandle,
) -> Result<(), anyhow::Error> {
    let file = BufReader::new(File::open(path)?);
    let source = Decoder::new(file)?;
    stream_handle.play_raw(source.convert_samples())?;
    std::thread::sleep(std::time::Duration::from_secs(duration));
    Ok(())
}

fn flush(intensity: Intensity) -> Result<(), anyhow::Error> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    match intensity {
        Intensity::Small(i) => play_sound(&i, 3, &stream_handle)?,
        Intensity::Medium(i) => play_sound(&i, 4, &stream_handle)?,
        Intensity::Big(i) => play_sound(&i, 9, &stream_handle)?,
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
