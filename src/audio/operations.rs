use std::ops::Div;
use std::ops::Sub;

use crate::time::has_sampling_frequency::HasSamplingFrequency;
use crate::time::milliseconds_to_samples;

use super::Audio;

impl Sub for Audio {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.merge_audio(other)
    }
}

impl Div for Audio {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        return self.overlap(other);
    }
}

#[allow(dead_code)]
impl Audio {
    fn match_length(&mut self, other: &mut Self) {
        self.match_sampling_frequencies(other);
        let len_self = self.samples.len();
        let len_other = other.samples.len();
        let length = len_self.max(len_other);
        self.samples.resize(length, 0.0);
        other.samples.resize(length, 0.0);
    }

    fn match_sampling_frequencies(&mut self, other: &mut Self) {
        let sampling_frequency = f64::max(
            self.get_sampling_frequency(),
            other.get_sampling_frequency(),
        );
        self.set_sampling_frequency(sampling_frequency);
        other.set_sampling_frequency(sampling_frequency);
    }

    pub fn merge_audio(mut self, mut other: Self) -> Self {
        self.match_sampling_frequencies(&mut other);
        let new_values = [&self.samples[..], &other.samples[..]].concat();
        return Audio {
            samples: new_values,
            sampling_frequency: Some(self.get_sampling_frequency()),
        };
    }

    pub fn merge<T>(self, wave: T) -> Self
    where
        T: Into<Audio>,
    {
        let other = wave.into();
        return self.merge_audio(other);
    }

    pub fn overlap(mut self, mut other: Self) -> Self {
        self.match_length(&mut other);
        let overlapped_samples = self
            .samples
            .into_iter()
            .zip(other.samples.into_iter())
            .map(|(sample_self, sample_other)| sample_self + sample_other)
            .collect();
        return Audio {
            samples: overlapped_samples,
            sampling_frequency: self.sampling_frequency,
        };
    }

    pub fn reverse(mut self) {
        self.samples.reverse();
    }

    pub fn sample_right_pad(&mut self, ammount: usize) {
        self.samples.resize(self.samples.len() + ammount, 0.0);
    }

    pub fn milliseconds_right_pad(&mut self, time_interval: f64) {
        let sampling_frequency = self.sampling_frequency.unwrap_or(0.0);
        let ammount = milliseconds_to_samples(sampling_frequency, time_interval);
        self.sample_right_pad(ammount);
    }

    pub fn sample_left_pad(&mut self, ammount: usize) {
        self.sample_right_pad(ammount);
        self.samples.rotate_right(ammount);
    }

    pub fn milliseconds_left_pad(&mut self, time_interval: f64) {
        let sampling_frequency = self.sampling_frequency.unwrap_or(0.0);
        let ammount = milliseconds_to_samples(sampling_frequency, time_interval);
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
        let sampling_frequency = self.sampling_frequency.unwrap_or(0.0);
        let index = milliseconds_to_samples(sampling_frequency, time_ms);
        return self.split_at_sample_index(index);
    }
}
