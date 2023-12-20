# Rust Waveform Generator

A simple Rust library for generating various waveforms.

## Features

- Generate different types of waveforms: Sine, Square, Triangle, Sawtooth, and Noise.
- Set waveform properties such as frequency, amplitude, DC offset, and waveform type.
- Process individual samples of the waveform.

## Usage

```rust
use waveform_generator::Waveform;

fn main() {
    // Create a sine waveform with default settings
    let mut wave = Waveform::default();

    // Set waveform properties
    wave.set_frequency(440.0);

    // Process a single sample of the waveform
    let sample = wave.process();

    println!("Generated sample: {}", sample);
}
```
