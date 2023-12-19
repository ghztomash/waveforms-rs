use textplots::{Chart, Plot, Shape};

fn main() {
    println!("y = sin(x) / x");

    Chart::default()
        .lineplot(&Shape::Continuous(Box::new(|x| x.sin() / x)))
        .display();
}
