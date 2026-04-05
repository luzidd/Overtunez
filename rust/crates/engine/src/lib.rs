pub mod partial;
pub mod wavetable;

use partial::Partial;
use wavetable::Wavetable;

pub struct Engine {
    wavetable: Wavetable,
    partial: Partial,
    sample_rate: f32,
}

impl Engine {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            wavetable: Wavetable::sine(),
            partial: Partial::new(440.0, 0.8, 0.0, sample_rate),
            sample_rate,
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.partial
            .set_frequency(self.partial.frequency, sample_rate);
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.partial.set_frequency(frequency, self.sample_rate);
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.partial.amplitude = amplitude;
    }

    pub fn set_phase_offset(&mut self, phase_offset: f32) {
        self.partial.phase_offset = phase_offset;
    }

    pub fn reset(&mut self) {
        self.partial.reset();
    }

    /// Process a block of audio, writing into the provided output buffer.
    /// Both channels receive the same mono signal.
    pub fn process(&mut self, left: &mut [f32], right: &mut [f32]) {
        for (l, r) in left.iter_mut().zip(right.iter_mut()) {
            let sample = self.partial.next_sample(&self.wavetable);
            *l = sample;
            *r = sample;
        }
    }
}
