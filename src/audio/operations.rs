use std::ops::Div;
use std::ops::Sub;

use crate::utils::build::Build;

use super::{AudioBuilder, Audio, ToAudio, InvalidAudio, InvalidAudioKind};

impl Sub for Audio {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match self.merge(other) {
            Ok(merged_audio) => merged_audio,
            Err(_merge_error) => {
                todo!("Once implemented, we'll have automatic upsampling")
            }
        }
    }
}

impl Div for Audio {
    type Output = Result<Self, InvalidAudio>;

    fn div(self, other: Self) -> Self::Output {
        return self.overlap(other);
    }
}

#[allow(dead_code)]
impl Audio {
    fn matching_sampling_frequency(&self, other: &Self) -> bool {
        return self.sampling_frequency == other.sampling_frequency;
    }

    pub fn merge(self, other: Self) -> Result<Self, InvalidAudio> {
        if !self.matching_sampling_frequency(&other) {
            return Err(InvalidAudio {
                kind: InvalidAudioKind::MismatchedSamplingFrequency,
            });
        }
        let new_values = [&self.samples[..], &other.samples[..]].concat();
        return Ok(AudioBuilder::new(new_values, self.sampling_frequency).finalize()?);
    }

    pub fn merge_wave<T>(self, wave: T) -> Result<Self, InvalidAudio>
    where
        T: ToAudio,
    {
        let other = wave.to_audio()?;
        return self.merge(other);
    }

    pub fn validate_overlap(&self, other: &Self) -> Result<(), Vec<InvalidAudio>> {
        let len_self = self.samples.len();
        let len_other = other.samples.len();
        let mut possible_errors: Vec<InvalidAudio> = vec![];
        if !&self.matching_sampling_frequency(&other) {
            possible_errors.push(InvalidAudio {
                kind: InvalidAudioKind::MismatchedSamplingFrequency,
            });
        }
        if len_self != len_other {
            possible_errors.push(InvalidAudio {
                kind: InvalidAudioKind::MismatchedLength,
            });
        }
        if !possible_errors.is_empty() {
            return Err(possible_errors);
        };
        return Ok(());
    }

    pub fn overlap(self, other: Self) -> Result<Self, InvalidAudio> {
        let len_self = self.samples.len();
        let len_other = other.samples.len();
        if !&self.matching_sampling_frequency(&other) {
            return Err(InvalidAudio {
                kind: InvalidAudioKind::MismatchedSamplingFrequency,
            });
        }
        if len_self != len_other {
            return Err(InvalidAudio {
                kind: InvalidAudioKind::MismatchedLength,
            });
        }
        let overlapped_samples = self
            .samples
            .into_iter()
            .zip(other.samples.into_iter())
            .map(|(sample_self, sample_other)| sample_self + sample_other)
            .collect();
        return Ok(Audio {
            samples: overlapped_samples,
            sampling_frequency: self.sampling_frequency,
        });
    }

    pub fn reverse(mut self) {
        self.samples.reverse();
    }

    pub fn sample_right_pad(&mut self, ammount: usize) {
        self.samples.resize(self.samples.len() + ammount, 0.0);
    }

    pub fn milliseconds_right_pad(&mut self, time_interval: f64) {
        let ammount = Audio::milliseconds_to_samples(self.sampling_frequency, time_interval);
        self.sample_right_pad(ammount);
    }

    pub fn sample_left_pad(&mut self, ammount: usize) {
        self.sample_right_pad(ammount);
        self.samples.rotate_right(ammount);
    }

    pub fn milliseconds_left_pad(&mut self, time_interval: f64) {
        let ammount = Audio::milliseconds_to_samples(self.sampling_frequency, time_interval);
        self.sample_left_pad(ammount);
    }

    pub fn split_at_sample_index(self, sample_index: usize) -> (Self, Self) {
        let mut earlier = self.samples;
        let later = if sample_index > earlier.len() {
            vec![]
        } else {
            earlier.split_off(sample_index)
        };
        let sampling_frequency = self.sampling_frequency;
        let earlier_audio = Audio {
            samples: earlier,
            sampling_frequency,
        };
        let later_audio = Audio {
            samples: later,
            sampling_frequency,
        };
        return (earlier_audio, later_audio);
    }

    pub fn split_at_time_ms(self, time_ms: f64) -> (Self, Self) {
        let index = Self::milliseconds_to_samples(self.sampling_frequency, time_ms);
        return self.split_at_sample_index(index);
    }
}

