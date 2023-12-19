pub mod waveforms;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut wave = waveforms::Waveform::new(44100.0, 1.0);
        let value = wave.process();
        assert_eq!(value, 0.0);
    }
}
