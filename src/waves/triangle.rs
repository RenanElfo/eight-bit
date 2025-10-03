use std::f64::consts::PI;

use builder_derive_macro::Setters;

use crate::utils::build::Build;
use crate::audio::{Audio, AudioBuilder, InvalidAudio, traits::ToAudio};
use crate::tone;

use super::{InvalidWaveForm, InvalidWaveFormKind};

#[derive(Clone, Debug, PartialEq, Setters)]
pub struct TriangleBuilder {
    tone: tone::Tone,
    amplitude: f64,
    rad_phase: f64,
    duration_ms: f64,
    sampling_frequency: f64,
}

impl Default for TriangleBuilder {
    fn default() -> Self {
        return Self {
            tone: tone::Tone::default(),
            amplitude: 1.0,
            rad_phase: 0.0,
            duration_ms: 0.0,
            sampling_frequency: 44100_f64,
        };
    }
}

#[allow(dead_code)]
impl TriangleBuilder {
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

    pub fn finalize(self) -> Result<Triangle, InvalidWaveForm> {
        if let Result::Err(error) = self.validate() {
            return Err(error[0].clone());
        }
        return Ok(Triangle {
            tone: self.tone,
            amplitude: self.amplitude,
            rad_phase: self.rad_phase,
            duration_ms: self.duration_ms,
            sampling_frequency: self.sampling_frequency,
            sample_index: 0,
        });
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Triangle {
    tone: tone::Tone,
    amplitude: f64,
    rad_phase: f64,
    duration_ms: f64,
    sampling_frequency: f64,
    sample_index: usize,
}

impl Triangle {
    fn number_of_samples(&self) -> usize {
        return Audio::milliseconds_to_samples(self.sampling_frequency, self.duration_ms);
    }
}

impl Iterator for Triangle {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sample_index >= self.number_of_samples() {
            return None;
        }
        let frequency: f64 = self.tone.into();
        let period: f64 = 1.0 / frequency;
        let time = Audio::samples_to_seconds(self.sampling_frequency, self.sample_index);
        let time_with_phase = time + self.rad_phase * period / (2.0 * PI);
        let modulus = |x, m| ((x % m) + m) % m;
        self.sample_index = self.sample_index + 1;
        return Some(
            4.0 * self.amplitude
                * frequency
                * f64::abs(modulus(time_with_phase - period / 4.0, period) - period / 2.0)
                - self.amplitude,
        );
    }
}

impl ToAudio for Triangle {
    fn to_audio(self) -> Result<Audio, InvalidAudio> {
        let sampling_frequency = self.sampling_frequency;
        let builder = AudioBuilder::new(self.collect(), sampling_frequency);
        return builder.finalize();
    }
}
