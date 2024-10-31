use std::fs;
use std::path::PathBuf;
use byteorder::{ByteOrder, LittleEndian};
use hound::WavReader;

pub fn process_texture(src: &PathBuf) -> Option<Vec<u8>> {
    match image::open(src) {
        Ok(tex) => {
            let raw_data = tex.to_rgba8().into_raw();
            Some(raw_data)
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            None
        }
    }
}

pub fn process_audio(src: &PathBuf) -> Option<Vec<u8>> {
    let mut reader = WavReader::open(src).expect("Failed to open WAV file.");
    let spec = reader.spec();

    if (spec.bits_per_sample != 32) {
        eprintln!("XPak currently only supports 32-bit float encoded samples.");
        return None;
    }

    // Stereo audio data is stored as interleaved samples in pak files
    let samples: Vec<f32> = reader.samples::<f32>()
        .map(|s| s.expect("Failed to read sample"))
        .collect();

    let mut byte_array: Vec<u8> = Vec::with_capacity(samples.len() * size_of::<f32>());
    for &sample in &samples {
        let mut bytes = [0u8; 4];
        // All audio sample data is stored in little endian byte order in pak files
        LittleEndian::write_f32(&mut bytes, sample);
        byte_array.extend_from_slice(&bytes);
    }

    Some(byte_array)
}
pub fn process_data(src: &PathBuf) -> Option<Vec<u8>> {
    match fs::read(src) {
        Ok(data) => Some(data),
        Err(e) => {
            eprintln!("Error: {}", e);
            None
        }
    }
}