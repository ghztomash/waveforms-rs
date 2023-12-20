# Rust Waveform Generator

A simple Rust library for generating various waveforms.

## Features

- Generate different types of waveforms: Sine, Square, Triangle, Sawtooth, and Noise.
- Set waveform properties such as frequency, amplitude, DC offset, and waveform type.
- Process individual samples of the waveform.

## Usage

```rust
use waveforms_rs::Waveform;

fn main() {
    // Create a sine waveform with default settings
    let mut wave = Waveform::default();

    // Process a couple samples of the waveform
    let samples = [
        wave.process(),
        wave.process(),
        wave.process(),
        wave.process(),
    ];

    println!("Generated samples: {:?}", samples);
}
```

## Running the examples

`cargo run --example basic`
`cargo run --example plot`
