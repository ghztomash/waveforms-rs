use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::f32::consts::PI;
use std::fmt;
use std::time::{Duration, SystemTime};

const TWO_PI: f32 = 2.0 * PI;

#[derive(Debug)]
pub struct Waveform {
    sample_rate: f32,
    frequency: f32,
    amplitude: f32,
    phase: f32,
    phase_increment: f32,
    phase_offset: f32,
    dc_offset: f32,
    waveform_type: WaveformType,
    rng: SmallRng,
}

#[derive(Debug)]
pub enum WaveformType {
    Sine,
    Square,
    Triangle,
    Sawtooth,
    Noise,
}

impl fmt::Display for WaveformType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WaveformType::Sine => write!(f, "Sine"),
            WaveformType::Square => write!(f, "Square"),
            WaveformType::Triangle => write!(f, "Triangle"),
            WaveformType::Sawtooth => write!(f, "Sawtooth"),
            WaveformType::Noise => write!(f, "Noise"),
        }
    }
}

impl TryFrom<u8> for WaveformType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(WaveformType::Sine),
            1 => Ok(WaveformType::Square),
            2 => Ok(WaveformType::Triangle),
            3 => Ok(WaveformType::Sawtooth),
            4 => Ok(WaveformType::Noise),
            _ => Err(()),
        }
    }
}

impl Default for Waveform {
    fn default() -> Self {
        return Waveform::new(44100.0, 440.0);
    }
}

impl Waveform {
    pub fn new(sample_rate: f32, frequency: f32) -> Self {
        let seed = SystemTime::now()
            .elapsed()
            .unwrap_or(Duration::from_millis(0))
            .as_millis() as u64;
        let mut wave = Waveform {
            sample_rate,
            frequency: 0.0,
            amplitude: 0.0,
            phase: 0.0,
            phase_increment: 0.0,
            phase_offset: 0.0,
            dc_offset: 0.0,
            waveform_type: WaveformType::Sine,
            rng: SmallRng::seed_from_u64(seed),
        };
        wave.set_frequency(frequency);
        wave.set_amplitude(1.0);
        return wave;
    }

    pub fn new_with_type(sample_rate: f32, frequency: f32, waveform_type: WaveformType) -> Self {
        let mut wave = Waveform::new(sample_rate, frequency);
        wave.set_frequency(frequency);
        wave.set_amplitude(1.0);
        wave.set_waveform_type(waveform_type);
        return wave;
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        if frequency < 0.0 {
            self.frequency = 0.0;
        } else if frequency > self.sample_rate / 2.0 {
            self.frequency = self.sample_rate / 2.0;
        } else {
            self.frequency = frequency;
        }

        self.phase_increment = (self.frequency * TWO_PI) / self.sample_rate;
    }

    pub fn frequency(&self) -> f32 {
        self.frequency
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }

    pub fn amplitude(&self) -> f32 {
        self.amplitude
    }

    pub fn set_dc_offset(&mut self, dc_offset: f32) {
        self.dc_offset = dc_offset;
    }

    pub fn dc_offset(&self) -> f32 {
        self.dc_offset
    }

    pub fn set_waveform_type(&mut self, waveform_type: WaveformType) {
        self.waveform_type = waveform_type;
    }

    pub fn set_phase_offset(&mut self, phase_offset: f32) {
        if phase_offset > TWO_PI {
            self.phase_offset = phase_offset % TWO_PI;
        } else if phase_offset < -TWO_PI {
            self.phase_offset = -(-phase_offset % TWO_PI);
        } else {
            self.phase_offset = phase_offset;
        }
    }

    pub fn phase_offset(&self) -> f32 {
        self.phase_offset
    }

    pub fn reset(&mut self) {
        self.phase = 0.0;
    }

    /// Process a single sample of the waveform.
    pub fn process(&mut self) -> f32 {
        let sample;

        match self.waveform_type {
            WaveformType::Sine => {
                sample = self.phase.sin();
            }
            WaveformType::Square => {
                if self.phase < PI {
                    sample = 1.0;
                } else {
                    sample = -1.0;
                }
            }
            WaveformType::Triangle => {
                if self.phase < PI {
                    sample = -1.0 + (2.0 * self.phase / PI);
                } else {
                    sample = 3.0 - (2.0 * self.phase / PI);
                }
            }
            WaveformType::Sawtooth => {
                sample = -1.0 + (2.0 * self.phase / TWO_PI);
            }
            WaveformType::Noise => {
                sample = self.rng.gen::<f32>() * 2.0 - 1.0;
            }
        }

        self.phase = (self.phase + self.phase_increment) % TWO_PI;
        return self.dc_offset + (sample * self.amplitude);
    }

    pub fn waveform_type(&self) -> &WaveformType {
        return &self.waveform_type;
    }

    pub fn waveform_name(&self) -> String {
        return format!("{}", self.waveform_type);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut wave = Waveform::default();
        let value = wave.process();
        assert_eq!(value, 0.0);
    }

    #[test]
    fn test_gain() {
        let mut wave = Waveform::new(4.0, 1.0);
        wave.set_amplitude(0.0);

        let values = [
            wave.process(),
            wave.process(),
            wave.process(),
            wave.process(),
        ];
        assert_eq!(values, [0.0; 4]);

        wave.set_amplitude(2.0);

        let values = [
            wave.process(),
            wave.process(),
            wave.process(),
            wave.process(),
        ];

        let expected_values = [0.0, 2.0, 0.0, -2.0];

        for (actual, expected) in values.iter().zip(expected_values.iter()) {
            assert!((actual - expected) <= f32::EPSILON);
        }
    }

    #[test]
    fn test_sine() {
        let mut wave = Waveform::new(4.0, 1.0);

        let values = [
            wave.process(),
            wave.process(),
            wave.process(),
            wave.process(),
            wave.process(),
            wave.process(),
            wave.process(),
            wave.process(),
        ];

        let expected_values = [0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0];

        for (actual, expected) in values.iter().zip(expected_values.iter()) {
            assert!((actual - expected) <= f32::EPSILON);
        }
    }

    #[test]
    fn test_dc_offset() {
        let mut wave = Waveform::new(4.0, 1.0);
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

        wave.set_amplitude(1.0);
        wave.set_dc_offset(1.0);

        let values = [
            wave.process(),
            wave.process(),
            wave.process(),
            wave.process(),
        ];

        let expected_values = [1.0, 2.0, 1.0, 0.0];

        for (actual, expected) in values.iter().zip(expected_values.iter()) {
            assert!((actual - expected) <= f32::EPSILON);
        }
    }
}
