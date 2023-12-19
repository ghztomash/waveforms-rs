use rgb::RGB8;
use textplots::{Chart, ColorPlot, Shape};
use waveforms_rs::waveforms::Waveform;

fn main() {
    let mut wave = Waveform::default();

    let mut values = Vec::new();
    for i in 0..440 {
        values.push((i as f32, wave.process()));
    }

    println!("sin()");
    Chart::new(160, 80, 0.0, 441.0)
        .linecolorplot(&Shape::Lines(&values), RGB8 { r: 0, g: 0, b: 255 })
        .display();
}
