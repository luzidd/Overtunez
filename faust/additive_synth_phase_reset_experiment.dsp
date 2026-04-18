import("stdfaust.lib");

// --- Additive Synthesis with Per-Harmonic Decay ---
// Each harmonic has its own exponential decay envelope.
// Higher harmonics decay faster (proportional to harmonic index),
// so the timbre starts bright and becomes darker over time — like a plucked string.
//
// At the moment of the pluck (gate on):
//   All 128 harmonics sound → bright, full sawtooth-like spectrum
// After release:
//   Harmonic 128 dies almost instantly
//   Harmonic 64 dies in ~twice that time
//   ...
//   Harmonic 1 (fundamental) rings the longest
// Result: the sound naturally evolves from bright → dark → silence

N_HARMONICS = 192;

freq = hslider("freq", 55, 50, 2000, 0.1) : si.smoo;
gain = hslider("gain", 0.8, 0, 1, 0.01) : si.smoo;
gate = button("gate");

// Base decay time — harmonic 1 decays over this many seconds.
// Higher harmonics decay n× faster.
decay_time = hslider("decay", 0.4, 0.1, 5.0, 0.1);

// Per-harmonic envelope: attack/release envelope where release = decay_time / n
// Harmonic 1: release = 1.0s, Harmonic 2: 0.5s, ..., Harmonic 128: 0.0078s
harmonic_env(n) = en.ar(0.0, decay_time / (n + 1), gate);

// Rising-edge trigger: fires on the sample gate goes from 0 → 1
trig = gate > gate';

// Single master phasor for the fundamental — resets on trigger.
// All harmonics derive their phase from this, so integer relationships
// are always exact and never drift.
TSIZE = 1 << 16;
master_phase = (+(freq / ma.SR) : ma.frac) ~ *(1 - trig);

// Harmonic n's phase is just (n+1)× the master phase, wrapped to [0,1)
osc_h(n) = s0 + (s1 - s0) * frac
  with {
    idx  = ma.frac(master_phase * (n + 1)) * float(TSIZE);
    i0   = int(idx);
    i1   = (i0 + 1) % TSIZE;
    frac = idx - float(i0);
    s0   = rdtable(TSIZE, os.sinwaveform(TSIZE), i0);
    s1   = rdtable(TSIZE, os.sinwaveform(TSIZE), i1);
  };

// Each harmonic: wavetable sine at (n+1)× freq, scaled by 1/(n+1), with its own decay
harmonic(n) = osc_h(n) * (1.0 / (n + 1)) * harmonic_env(n);

additive = sum(n, N_HARMONICS, harmonic(n));

// Precomputed: 1 / sum(k=1..128, 1/k) = 1 / 5.4331...
normalize = 0.18403;

// Independent global damping envelope — controls how long the full signal sustains
master_decay = hslider("master_decay", 0.6, 0.1, 5.0, 0.1);
master_env = en.ar(0.0001, master_decay, gate);

// Wrap the entire signal in the master envelope
process = (additive * normalize * gain * master_env) <: (_, _);
