use std::default::Default;

use builder_derive_macro::Setters;

use crate::time::has_duration::HasDuration;
use crate::time::has_sampling_frequency::HasSamplingFrequency;
use crate::time::{milliseconds_to_samples, samples_to_milliseconds};
use crate::utils::build::Build;

pub mod basic_filters;
mod operations;
mod tests;
pub mod traits;
mod utils;
// use traits::ToAudio;

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
            sampling_frequency: Some(self.sampling_frequency),
        });
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Audio {
    samples: Vec<f64>,
    sampling_frequency: Option<f64>,
}

impl Audio {
    pub fn filter_audio<T>(self, filter: T) -> Audio where T: traits::FilterAudio {
        filter.filter(self)
    }
}

impl HasSamplingFrequency for Audio {
    fn get_sampling_frequency(&self) -> f64 {
        let sampling_frequency = self.sampling_frequency.unwrap_or(0.0);
        match sampling_frequency {
            0.0.. => sampling_frequency,
            _ => 0.0,
        }
    }

    fn set_sampling_frequency(&mut self, sampling_frequency: f64) {
        if self.sampling_frequency.is_none() {
            self.sampling_frequency = match sampling_frequency {
                0.0.. => Some(sampling_frequency),
                _ => None,
            }
        } else {
            if self.get_sampling_frequency() == sampling_frequency {
                return ();
            }
            println!("{:?}", sampling_frequency);
            todo!("TODO: do upsampling/downsampling");
        }
    }
}

impl HasDuration for Audio {
    fn get_duration_ms(&self) -> f64 {
        samples_to_milliseconds(self.get_sampling_frequency(), self.sample_length())
    }

    fn set_duration_ms(&mut self, duration_ms: f64) {
        let sampling_frequency = self.get_sampling_frequency();
        let new_length = milliseconds_to_samples(sampling_frequency, duration_ms);
        self.samples.resize(new_length, 0.0);
    }
}

impl Default for Audio {
    fn default() -> Self {
        return Audio {
            samples: vec![],
            sampling_frequency: None,
        };
    }
}

// impl Into<Audio> for Audio {
//     fn into(self) -> Audio {
//         return self;
//     }
// }
