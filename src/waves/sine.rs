use std::f64::consts::PI;

use builder_derive_macro::{Finalize, Setters};

use crate::audio::{Audio, AudioBuilder, InvalidAudio, ToAudio};
use crate::tone;

use super::{InvalidWaveForm, InvalidWaveFormKind};

#[derive(Clone, Debug, PartialEq, Setters)]
pub struct SineBuilder {
    tone: tone::Tone,
    amplitude: f64,
    duration_ms: f64,
    rad_phase: f64,
    sampling_frequency: f64,
}

impl Default for SineBuilder {
    fn default() -> Self {
        return Self {
            tone: tone::Tone::default(),
            amplitude: 1.0,
            duration_ms: 0.0,
            rad_phase: 0.0,
            sampling_frequency: 44100_f64,
        };
    }
}

#[allow(dead_code)]
impl SineBuilder {
    pub fn with_deg_phase(mut self, phase: f64) -> Self {
        self.rad_phase = PI * phase / 180.0;
        return self;
    }

    pub fn validate(&self) -> Result<(), Vec<InvalidWaveForm>> {
        let mut possible_errors: Vec<InvalidWaveForm> = vec![];
        if self.duration_ms < 0.0 {
            possible_errors.push(InvalidWaveForm {
                kind: InvalidWaveFormKind::NegativeDuration,
            });
        }
        if !possible_errors.is_empty() {
            return Err(possible_errors);
        };
        return Ok(());
    }

    pub fn finalize(self) -> Result<Sine, InvalidWaveForm> {
        if let Result::Err(error) = self.validate() {
            return Err(error[0].clone());
        }
        return Ok(Sine {
            tone: self.tone,
            amplitude: self.amplitude,
            duration_ms: self.duration_ms,
            rad_phase: self.rad_phase,
            sampling_frequency: self.sampling_frequency,
        });
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Sine {
    tone: tone::Tone,
    amplitude: f64,
    duration_ms: f64,
    rad_phase: f64,
    sampling_frequency: f64,
}

impl ToAudio for Sine {
    fn to_audio(self) -> Result<Audio, InvalidAudio> {
        let frequency: f64 = self.tone.into();
        let number_of_samples =
            Audio::milliseconds_to_samples(self.sampling_frequency, self.duration_ms);
        let indices = 0..number_of_samples;
        let samples: Vec<f64> = indices
            .into_iter()
            .map(|sample_index| {
                let time = Audio::samples_to_seconds(self.sampling_frequency, sample_index);
                return self.amplitude * (2.0 * PI * frequency * time + self.rad_phase).sin();
            })
            .collect();
        let builder = AudioBuilder::new(samples, self.sampling_frequency);
        return builder.finalize();
    }
}
