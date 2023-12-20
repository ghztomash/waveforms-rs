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
