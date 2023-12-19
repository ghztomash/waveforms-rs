const TWO_PI: f32 = 2.0 * std::f32::consts::PI;

pub struct Waveform {
    sample_rate: f32,
    frequency: f32,
    amplitude: f32,
    phase: f32,
    phase_increment: f32,
    phase_offset: f32,
    dc_offset: f32,
    running: bool,
    waveform_type: WaveformType,
}

pub enum WaveformType {
    Sine,
    Square,
    Triangle,
    Sawtooth,
}

impl Default for Waveform {
    fn default() -> Self {
        return Waveform::new(44100.0, 440.0);
    }
}

impl Waveform {
    pub fn new(sample_rate: f32, frequency: f32) -> Self {
        let mut wave = Waveform {
            sample_rate,
            frequency: 0.0,
            amplitude: 0.0,
            phase: 0.0,
            phase_increment: 0.0,
            phase_offset: 0.0,
            dc_offset: 0.0,
            running: false,
            waveform_type: WaveformType::Sine,
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

    pub fn set_amplitude(&mut self, amplitude: f32) {
            self.amplitude = amplitude;
    }

    pub fn set_dc_offset(&mut self, dc_offset: f32) {
            self.dc_offset = dc_offset;
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

    pub fn process(&mut self) -> f32 {
        let sample = self.amplitude * self.phase.sin();
        self.phase = (self.phase + self.phase_increment) % TWO_PI;
        return sample;
    }
}