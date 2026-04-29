

use colored::Colorize;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = if let Some(arg_one) = env::args().nth(1) {
        arg_one
    } else {
        "klechtch.wav".to_string()
    };

    let mut reader = hound::WavReader::open(&file_name)?;
    println!(
        "Reading a wav file: channels {}, sample rate {}, bps {}.",
        reader.spec().channels,
        reader.spec().sample_rate,
        reader.spec().bits_per_sample
    );

    // Convert the samples from i16 to f64.
    let samples: Vec<f64> = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f64 / 32767.0f64)
        .collect();
    println!(
        "Read {} samples with {} bytes of message data.",
        samples.len(),
        samples.len() / 1600
    );

    let byte_chunks = samples.as_chunks::<1600>();
    if !byte_chunks.1.is_empty() {
        println!("There are extra samples!");
    }

    let mut s = String::new();

    for chunk in byte_chunks.0 {
        let bit_samples = chunk.as_chunks::<160>().0;

        let mut byte = 0u8;
        // Skip the control bits, bit 0 and bit 9.
        for samples in bit_samples[1..9].iter().rev() {
            let power = tone_power(samples, 2225.0, 48000.0);
            // println!("{}", power);
            if power > 600.0 {
                // Eperimentally determined! This value (600) has to be changed if the
                // sample rate is different.
                byte = (byte << 1) | 1u8;
            } else {
                byte <<= 1
            }
        }

        s.push(byte as char);
    }

    println!("The message is: {}", s.green().bold());
    Ok(())
}

fn tone_power(samples: &[f64; 160], f: f64, fs: f64) -> f64 {
    let mut p = 0f64;
    let mut q = 0f64;

    for (i, s) in samples.iter().enumerate() {
        let w = std::f64::consts::TAU * f * i as f64 / fs;
        p += s * w.cos();
        q += s * w.sin();
    }

    p * p + q * q
}
