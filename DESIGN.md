# Design

## Design Goal

The default UX should be similar to Serum, but with a modular signal chain and more control over partials. The design prioritizes performance, ensuring smooth operation even with many partials and effects.

Routing uses a node-based system similar to the Grid in Bitwig Studio, where modules can be connected to create custom signal flows. For ergonomics, controls can be mapped to a customizable "dashboard" view for quick access to key parameters without navigating the node graph. This gives users both a flexible node-based system for complex sound design and a streamlined dashboard for quick adjustments.

## Signal Chain Overview

The modular signal chain lets users create multiple additive engines, choose and order effects, and control how partials are modulated.

- Additive engines
  - Partials
  - Modifiers
    - Amplitude
    - Phase
    - Frequency
- Effects
  - EQ
  - Waveshaping / Saturation
- Mixer

## Modules

### Additive Engines

Additive engines generate sound by summing multiple sine waves (partials), each modulatable in real-time.

#### Partials

Each partial is defined by its frequency, amplitude, and phase. Engines support up to 256 partials. Modulation sources include LFOs, envelopes, and functions.

#### Modifiers

Modifiers provide real-time control over partials:
- Amplitude: Volume of each partial.
- Phase: Phase offset, useful for timbral effects.
- Frequency: Pitch modulation for vibrato and other effects.

### Effects

Built-in effects include an EQ for frequency shaping and waveshaping/saturation for harmonic content and distortion. The effects chain order is customizable.

### Mixer

The mixer blends additive engine outputs with effects, providing level, panning, and other mix controls.

## Inspiration

- Serum's intuitive interface and powerful sound design capabilities.
- Bitwig Studio's Grid for flexible, visual routing.
  - Bitwigs Poly Grid based factory devices like Polymer and Filter+ which are node-based but expose a more traditional interface for quick access to parameters.
