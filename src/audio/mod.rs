use std::default::Default;

use builder_derive_macro::Setters;

use crate::utils::build::Build;

mod operations;
mod utils;
mod tests;
pub mod traits;
use traits::ToAudio;

const DEFAULT_SAMPLING_FREQUENCY: f64 = 0.0_f64;

#[derive(Clone, Debug, PartialEq)]
pub enum InvalidAudioKind {
    NanSamples,
    InfiniteValuedSamples,
    NegativeSamplingFrequency,
    MismatchedSamplingFrequency,
    MismatchedLength,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InvalidAudio {
    kind: InvalidAudioKind,
}

#[derive(Clone, Debug, PartialEq, Setters)]
pub struct AudioBuilder {
    samples: Vec<f64>,
    sampling_frequency: f64,
}

impl Default for AudioBuilder {
    fn default() -> Self {
        return AudioBuilder {
            samples: Vec::default(),
            sampling_frequency: DEFAULT_SAMPLING_FREQUENCY,
        };
    }
}

#[allow(dead_code)]
impl AudioBuilder {
    pub fn with_length(mut self, length: usize) -> Self {
        self.samples = vec![0.0; length];
        return self;
    }
}

#[allow(dead_code)]
impl Build for AudioBuilder {
    type Output = Audio;
    type Error = InvalidAudio;

    fn validate(&self) -> Result<(), Vec<InvalidAudio>> {
        let mut possible_errors: Vec<InvalidAudio> = vec![];
        if self.sampling_frequency < 0.0 {
            possible_errors.push(InvalidAudio {
                kind: InvalidAudioKind::NegativeSamplingFrequency,
            });
        }
        if self.samples.iter().any(|sample| sample.is_nan()) {
            possible_errors.push(InvalidAudio {
                kind: InvalidAudioKind::NanSamples,
            });
        }
        if self.samples.iter().any(|sample| sample.is_infinite()) {
            possible_errors.push(InvalidAudio {
                kind: InvalidAudioKind::InfiniteValuedSamples,
            });
        }
        if !possible_errors.is_empty() {
            return Err(possible_errors);
        }
        return Ok(());
    }

    fn finalize(self) -> Result<Audio, InvalidAudio> {
        if let Result::Err(errors) = self.validate() {
            return Err(errors[0].clone());
        }
        return Ok(Audio {
            samples: self.samples,
            sampling_frequency: self.sampling_frequency,
        });
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Audio {
    samples: Vec<f64>,
    sampling_frequency: f64,
}

impl Default for Audio {
    fn default() -> Self {
        return AudioBuilder::default()
            .finalize()
            .expect("Default AudioBuilder should produce valid Audio");
    }
}

// Getters
#[allow(dead_code)]
impl Audio {
    pub fn get_samples(self) -> Vec<f64> {
        return self.samples;
    }

    pub fn get_sampling_frequency(&self) -> f64 {
        return self.sampling_frequency;
    }
}

impl ToAudio for Audio {
    fn to_audio(self) -> Result<Audio, InvalidAudio> {
        return Ok(self);
    }
}
