use std::f64::consts::TAU;

pub const TABLE_SIZE: usize = 2048;

pub struct Wavetable {
    table: [f32; TABLE_SIZE],
}

impl Wavetable {
    pub fn sine() -> Self {
        let mut table: [f32; TABLE_SIZE] = [0.0; TABLE_SIZE]; // precompute one cycle of a sine wave
        for i in 0..TABLE_SIZE {
            let phase = i as f64 / TABLE_SIZE as f64;
            table[i] = (phase * TAU).sin() as f32; // TAU is 2π, so this maps phase [0, 1) to [0, 2π)
        }
        Self { table }
    }

    /// Look up a value from the wavetable using linear interpolation.
    ///
    /// `phase` is in [0.0, 1.0), representing one full cycle.
    #[inline]
    pub fn lookup(&self, phase: f64) -> f32 {
        let pos = phase * TABLE_SIZE as f64;
        let index = pos as usize; // rounded down index
        let frac = pos - index as f64;

        // Use modulo to wrap around the table, and linear interpolation for smooth output
        // a is the current waveform value, b is the next waveform value (wrapping around), and frac is the fractional part for interpolation
        let a = self.table[index % TABLE_SIZE];
        let b = self.table[(index + 1) % TABLE_SIZE];

        // Linear interpolation
        a + ((b - a) * frac as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sine_wavetable_endpoints() {
        let wt = Wavetable::sine();
        // sin(0) = 0
        assert!(wt.lookup(0.0).abs() < 1e-6);
        // sin(π/2) = 1, which is phase 0.25
        assert!((wt.lookup(0.25) - 1.0).abs() < 1e-3);
        // sin(π) = 0, which is phase 0.5
        assert!(wt.lookup(0.5).abs() < 1e-3);
        // sin(3π/2) = -1, which is phase 0.75
        assert!((wt.lookup(0.75) + 1.0).abs() < 1e-3);
    }
}
