pub mod waveforms;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut wave = waveforms::Waveform::new(44100.0, 1.0);
        assert_eq!(1.0, 0.0);
    }
}
