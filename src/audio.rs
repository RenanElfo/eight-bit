use std::default::Default;
use std::ops::Div;
use std::ops::Sub;
use std::result::Result;

const DEFAULT_SAMPLING_FREQUENCY: f64 = 44100_f64;

#[derive(Clone, Debug, PartialEq)]
pub enum InvalidAudioKind {
    NegativeSamplingFrequency,
    MismatchedSamplingFrequency,
    MismatchedLength,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InvalidAudio {
    kind: InvalidAudioKind,
}

#[derive(Clone, Debug, PartialEq)]
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
    pub fn new(values: Vec<f64>, sampling_frequency: f64) -> Self {
        return AudioBuilder {
            samples: values,
            sampling_frequency,
        };
    }

    pub fn with_length(mut self, length: usize) -> Self {
        self.samples = vec![0.0; length];
        return self;
    }

    pub fn with_sampling_frequency(mut self, sampling_frequency: f64) -> Self {
        self.sampling_frequency = sampling_frequency;
        return self;
    }

    pub fn validate(&self) -> Result<(), InvalidAudio> {
        if self.sampling_frequency < 0.0 {
            return Err(InvalidAudio {
                kind: InvalidAudioKind::NegativeSamplingFrequency,
            });
        }
        return Ok(());
    }

    pub fn finalize(self) -> Result<Audio, InvalidAudio> {
        self.validate()?;
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
            .expect("Default Audio should be valid");
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

    pub fn samples_to_milliseconds(sampling_frequency: f64, ammount: usize) -> f64 {
        return (ammount as f64) * 1000_f64 / sampling_frequency;
    }

    pub fn milliseconds_to_samples(sampling_frequency: f64, time_interval: f64) -> usize {
        return ((time_interval / 1000_f64) * sampling_frequency) as usize;
    }

    pub fn sample_length(&self) -> usize {
        return self.samples.len();
    }

    pub fn milliseconds_length(&self) -> f64 {
        return Audio::samples_to_milliseconds(self.sampling_frequency, self.sample_length());
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

impl Sub for Audio {
    type Output = Result<Self, InvalidAudio>;

    fn sub(self, other: Self) -> Self::Output {
        return self.merge(other);
    }
}

impl Div for Audio {
    type Output = Result<Self, InvalidAudio>;

    fn div(self, other: Self) -> Self::Output {
        return self.overlap(other);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let x: AudioBuilder = AudioBuilder::default();
        let y: AudioBuilder = AudioBuilder {
            samples: vec![],
            sampling_frequency: 44100_f64,
        };
        assert_eq!(x, y);
    }

    #[test]
    fn test_builder_functions() {
        let sampling_frequency = 8192_f64;
        let x: AudioBuilder = AudioBuilder::default()
            .with_length(3)
            .with_sampling_frequency(8192_f64);
        assert_eq!(x.samples, vec![0.0, 0.0, 0.0]);
        assert_eq!(x.sampling_frequency, sampling_frequency);
    }

    #[test]
    fn test_concatenation() {
        let x: Audio = AudioBuilder::new(vec![1.0, 2.0, 3.0], 44100_f64)
            .finalize()
            .unwrap();
        let y: Audio = AudioBuilder::new(vec![4.0, 5.0, 6.0], 44100_f64)
            .finalize()
            .unwrap();
        let z = x.clone().merge(y.clone()).unwrap();
        assert_eq!(
            z,
            AudioBuilder::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 44100_f64)
                .finalize()
                .unwrap()
        );
        let w = (x - y).unwrap();
        assert_eq!(z, w);
    }

    #[test]
    fn test_overlapping() {
        let x: Audio = AudioBuilder::new(vec![1.0], 44100_f64).finalize().unwrap();
        let y: Audio = AudioBuilder::new(vec![0.5], 44100_f64).finalize().unwrap();
        let z: Audio = x.clone().overlap(y.clone()).unwrap();
        assert_eq!(
            z,
            AudioBuilder::new(vec![1.5], 44100_f64).finalize().unwrap()
        );
        let w = x / y;
        assert_eq!(z, w.unwrap());
    }

    #[test]
    fn test_sample_right_padding() {
        let mut x: Audio = AudioBuilder::new(vec![1.0, 2.0, 3.0], 44100_f64)
            .finalize()
            .unwrap();
        x.sample_right_pad(2);
        assert_eq!(
            x,
            AudioBuilder::new(vec![1.0, 2.0, 3.0, 0.0, 0.0], 44100_f64)
                .finalize()
                .unwrap()
        );
    }

    #[test]
    fn test_milliseconds_right_padding() {
        let mut x: Audio = AudioBuilder::new(vec![1.0], 44100_f64).finalize().unwrap();
        x.milliseconds_right_pad(1000_f64);
        assert_eq!(x.samples.first().unwrap(), &1.0);
        assert_eq!(x.samples.len(), 44101);
    }

    #[test]
    fn test_sample_left_padding() {
        let mut x: Audio = AudioBuilder::new(vec![1.0, 2.0, 3.0], 44100_f64)
            .finalize()
            .unwrap();
        x.sample_left_pad(2);
        assert_eq!(
            x,
            AudioBuilder::new(vec![0.0, 0.0, 1.0, 2.0, 3.0], 44100_f64)
                .finalize()
                .unwrap()
        );
    }

    #[test]
    fn test_milliseconds_left_padding() {
        let mut x: Audio = AudioBuilder::new(vec![1.0], 44100_f64).finalize().unwrap();
        x.milliseconds_left_pad(1000_f64);
        assert_eq!(x.samples.last().unwrap(), &1.0);
        assert_eq!(x.samples.len(), 44101);
    }
}
