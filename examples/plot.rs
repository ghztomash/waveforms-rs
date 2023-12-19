use rgb::RGB8;
use textplots::{Chart, ColorPlot, Shape};
use waveforms_rs::Waveform;

fn main() {
    let mut wave = Waveform::new(1000.0, 20.0);

    let mut values = Vec::new();
    for i in 0..100 {
        values.push((i as f32, wave.process()));
    }

    let point_color = RGB8 {
        r: 255,
        g: 0,
        b: 255,
    };

    println!("sin()");
    Chart::new(160, 80, 0.0, 100.0)
        .linecolorplot(&Shape::Lines(&values), point_color)
        .display();
}
