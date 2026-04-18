# Overtunez

Additive synthesis engine for music production and sound design.
Written in Rust for performance and correctness.

## Features

- Additive synthesis with up to 256 partials
- Real-time modulation of partials
  - Amplitude
  - Phase
  - Frequency
- Built-in effects
- Multiple engines
- maybe: modular signal chain

## Formats

- CLAP
- VST3
- Standalone

## Project Structure

- `docs/`: documentation and design notes
- `faust/`: faust implementation of the synth engine
- `math/`: mathematical foundations and utilities
- `rust/`: source code for the engine, plugins and standalone synth

## Contributing

Contributions are welcome! Please see the [CONTRIBUTING.md](CONTRIBUTING.md) file for guidelines on how to contribute to this project.

## License

This project is licensed under the GPLv2 License. See the [LICENSE](LICENSE) file for details.
