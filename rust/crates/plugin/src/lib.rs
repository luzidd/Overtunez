use nih_plug::prelude::*;
use std::sync::Arc;

use overtunez_engine::Engine;

pub struct Overtunez {
    params: Arc<OvertunezParams>,
    engine: Engine,
}

#[derive(Params)]
struct OvertunezParams {
    #[id = "frequency"]
    frequency: FloatParam,
    #[id = "amplitude"]
    amplitude: FloatParam,
    #[id = "phase_offset"]
    phase_offset: FloatParam,
}

impl Default for OvertunezParams {
    fn default() -> Self {
        Self {
            frequency: FloatParam::new(
                "Frequency",
                440.0,
                FloatRange::Skewed {
                    min: 20.0,
                    max: 20000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit(" Hz")
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2)),
            amplitude: FloatParam::new(
                "Amplitude",
                0.8,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            phase_offset: FloatParam::new(
                "Phase Offset",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
        }
    }
}

impl Default for Overtunez {
    fn default() -> Self {
        Self {
            params: Arc::new(OvertunezParams::default()),
            engine: Engine::new(44100.0),
        }
    }
}

impl Plugin for Overtunez {
    const NAME: &'static str = "Overtunez";
    const VENDOR: &'static str = "";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: None,
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.engine.set_sample_rate(buffer_config.sample_rate);
        true
    }

    fn reset(&mut self) {
        self.engine.reset();
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        self.engine.set_frequency(self.params.frequency.smoothed.next());
        self.engine.set_amplitude(self.params.amplitude.smoothed.next());
        self.engine.set_phase_offset(self.params.phase_offset.smoothed.next());

        let (left, right) = buffer.as_slice().split_at_mut(1);
        self.engine.process(&mut left[0], &mut right[0]);

        ProcessStatus::Normal
    }
}

impl ClapPlugin for Overtunez {
    const CLAP_ID: &'static str = "com.overtunez.overtunez";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Additive synthesis engine");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Instrument,
        ClapFeature::Synthesizer,
        ClapFeature::Stereo,
    ];
}

impl Vst3Plugin for Overtunez {
    const VST3_CLASS_ID: [u8; 16] = *b"OvertunezPlugin!";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Synth,
    ];
}

nih_export_clap!(Overtunez);
nih_export_vst3!(Overtunez);
