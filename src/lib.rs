pub mod waveforms;

#[cfg(test)]
mod tests {
    use crate::waveforms::Waveform;

    #[test]
    fn test_basic() {
        let mut wave = Waveform::default();
        let value = wave.process();
        assert_eq!(value, 0.0);
    }

    #[test]
    fn test_gain() {
        let mut wave = Waveform::default();
        wave.set_amplitude(0.0);

        let values = [
            wave.process(),
            wave.process(),
            wave.process(),
            wave.process(),
        ];
        assert_eq!(values, [0.0; 4]);
    }

    #[test]
    fn test_dc_offset() {
        let mut wave = Waveform::default();
        wave.set_amplitude(0.0);
        wave.set_dc_offset(1.0);

        let values = [
            wave.process(),
            wave.process(),
            wave.process(),
            wave.process(),
        ];
        assert_eq!(values, [1.0; 4]);

        wave.set_dc_offset(-1.0);

        let values = [
            wave.process(),
            wave.process(),
            wave.process(),
            wave.process(),
        ];
        assert_eq!(values, [-1.0; 4]);
    }
}
