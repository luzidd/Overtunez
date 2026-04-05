use crate::wavetable::Wavetable;

pub struct Partial {
    pub frequency: f32,
    pub amplitude: f32,
    pub phase_offset: f32,
    phase: f64,
    phase_increment: f64,
}

impl Partial {
    pub fn new(
        frequency: f32,
        amplitude: f32,
        phase_offset: f32,
        sample_rate: f32) -> Self {
        Self {
            frequency,
            amplitude,
            phase_offset,
            phase: 0.0,
            phase_increment: frequency as f64 / sample_rate as f64,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32, sample_rate: f32) {
        self.frequency = frequency;
        self.phase_increment = frequency as f64 / sample_rate as f64;
    }

    pub fn reset(&mut self) {
        self.phase = 0.0;
    }

    /// Generate the next sample from this partial using the given wavetable.
    #[inline]
    pub fn next_sample(&mut self, wavetable: &Wavetable) -> f32 {
        let phase = (self.phase + self.phase_offset as f64).fract();
        // Ensure phase is positive (fract can return negative for negative offsets)
        let phase = if phase < 0.0 { phase + 1.0 } else { phase };

        let sample = wavetable.lookup(phase) * self.amplitude;

        self.phase += self.phase_increment;
        // Keep phase in [0, 1) to avoid precision loss over time
        // using arithmetic instead of modulus for better performance, since phase_increment is small and phase grows slowly
        self.phase -= self.phase.floor();

        sample
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_produces_nonzero_output() {
        let wt = Wavetable::sine();
        let mut p = Partial::new(440.0, 1.0, 0.0, 44100.0);

        // After a few samples, we should get non-zero output
        let mut max = 0.0f32;
        for _ in 0..100 {
            let s = p.next_sample(&wt);
            max = max.max(s.abs());
        }
        assert!(max > 0.1);
    }

    #[test]
    fn zero_amplitude_is_silent() {
        let wt = Wavetable::sine();
        let mut p = Partial::new(440.0, 0.0, 0.0, 44100.0);

        for _ in 0..100 {
            assert_eq!(p.next_sample(&wt), 0.0);
        }
    }
}
