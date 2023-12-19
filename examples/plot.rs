use rgb::RGB8;
use textplots::{Chart, ColorPlot, Shape};
use waveforms_rs::{Waveform, WaveformType};

fn main() {
    let mut wave = Waveform::new(1000.0, 20.0);

    let mut sin_values = Vec::new();
    for i in 0..100 {
        sin_values.push((i as f32, wave.process()));
    }

    wave.set_waveform_type(WaveformType::Sawtooth);
    wave.set_amplitude(0.5);
    wave.set_dc_offset(0.5);
    wave.reset();

    let mut saw_values = Vec::new();
    for i in 0..100 {
        saw_values.push((i as f32, wave.process()));
    }

    println!("Waveforms");
    Chart::new(160, 80, 0.0, 100.0)
        .linecolorplot(
            &Shape::Lines(&saw_values),
            RGB8 {
                r: 0,
                g: 255,
                b: 255,
            },
        )
        .linecolorplot(
            &Shape::Lines(&sin_values),
            RGB8 {
                r: 255,
                g: 0,
                b: 255,
            },
        )
        .display();
}
