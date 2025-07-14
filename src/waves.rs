use std::default::Default;
use std::ops::Div;
use std::ops::Sub;
use std::result::Result;

const DEFAULT_AMPLITUDE: f64 = 1_f64;
const DEFAULT_SAMPLING_FREQUENCY: f64 = 44100_f64;

#[derive(Clone, Debug, PartialEq)]
pub struct Wave {
    values: Vec<i16>,
    amplitude: f64,
    sampling_frequency: f64,
}

impl Default for Wave {
    fn default() -> Self {
        return Wave {
            values: Vec::default(),
            amplitude: DEFAULT_AMPLITUDE,
            sampling_frequency: DEFAULT_SAMPLING_FREQUENCY,
        };
    }
}

impl Wave {
    pub fn new(values: Vec<i16>, amplitude: f64, sampling_frequency: f64) -> Self {
        return Wave {
            values,
            amplitude,
            sampling_frequency,
        };
    }

    pub fn with_length(mut self, length: usize) -> Self {
        self.values = vec![0; length];
        self.amplitude = 0_f64;
        return self;
    }

    pub fn with_amplitude(mut self, amplitude: f64) -> Self {
        self.amplitude = amplitude;
        return self;
    }

    pub fn with_sampling_frequency(mut self, sampling_frequency: f64) -> Self {
        self.sampling_frequency = sampling_frequency;
        return self;
    }

    pub fn merge(&self, other: &Self) -> Result<Self, &'static str> {
        if self.sampling_frequency != other.sampling_frequency {
            return Err("Can't merge waves with different sampling frequencies");
        }
        let new_values = [&self.values[..], &other.values[..]].concat();
        return Ok(Wave::new(new_values, 1_f64, self.sampling_frequency));
    }

    pub fn overlap(&self, other: &Self) -> Result<Self, &'static str> {
        let len_self = self.values.len();
        let len_other = other.values.len();
        if len_self != len_other {
            return Err("values don't have the same length");
        }
        return Ok(Wave::new(vec![0], 1_f64, 44100_f64));
    }

    pub fn sample_right_pad(&mut self, ammount: usize) {
        self.values.resize(self.values.len() + ammount, 0);
    }

    pub fn milliseconds_right_pad(&mut self, time_interval: f64) {
        let ammount = self.milliseconds_to_samples(time_interval);
        self.sample_right_pad(ammount);
    }

    pub fn sample_left_pad(&mut self, ammount: usize) {
        self.sample_right_pad(ammount);
        self.values.rotate_right(ammount);
    }

    pub fn milliseconds_left_pad(&mut self, time_interval: f64) {
        let ammount = self.milliseconds_to_samples(time_interval);
        self.sample_left_pad(ammount);
    }

    pub fn samples_to_milliseconds(&self, ammount: usize) -> f64 {
        return (ammount as f64) * 1000_f64 / self.sampling_frequency;
    }

    pub fn milliseconds_to_samples(&self, time_interval: f64) -> usize {
        return ((time_interval / 1000_f64) * self.sampling_frequency) as usize;
    }

    pub fn sample_length(&self) -> usize {
        return self.values.len();
    }

    pub fn milliseconds_length(&self) -> f64 {
        return self.samples_to_milliseconds(self.sample_length());
    }
}

impl Sub for Wave {
    type Output = Result<Self, &'static str>;

    fn sub(self, other: Self) -> Self::Output {
        return self.merge(&other);
    }
}

impl Div for Wave {
    type Output = Result<Self, &'static str>;

    fn div(self, other: Self) -> Self::Output {
        return self.overlap(&other);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let x: Wave = Wave::default();
        let y: Wave = Wave {
            values: vec![],
            amplitude: 1_f64,
            sampling_frequency: 44100_f64,
        };
        assert_eq!(x, y);
    }

    #[test]
    fn test_builder_functions() {
        let amplitude = 2_f64;
        let sampling_frequency = 8192_f64;
        let x: Wave = Wave::default()
            .with_length(3)
            .with_amplitude(2_f64)
            .with_sampling_frequency(8192_f64);
        assert_eq!(x.values, vec![0, 0, 0]);
        assert_eq!(x.amplitude, amplitude);
        assert_eq!(x.sampling_frequency, sampling_frequency);
    }

    #[test]
    fn test_concatenation() {
        let x: Wave = Wave::new(vec![1, 2, 3], 1_f64, 44100_f64);
        let y: Wave = Wave::new(vec![4, 5, 6], 1_f64, 44100_f64);
        let z = x.merge(&y).unwrap();
        assert_eq!(z, Wave::new(vec![1, 2, 3, 4, 5, 6], 1_f64, 44100_f64));
        let w = (x - y).unwrap();
        assert_eq!(z, w);
    }

    #[test]
    fn test_overlapping() {
        let x: Wave = Wave::new(vec![0], 1_f64, 44100_f64);
        let y: Wave = Wave::new(vec![0], 1_f64, 44100_f64);
        let z: Wave = x.overlap(&y).unwrap();
        assert_eq!(z, Wave::new(vec![0], 1_f64, 44100_f64));
        let w = x / y;
        assert_eq!(z, w.unwrap());
    }

    #[test]
    fn test_sample_right_padding() {
        let mut x: Wave = Wave::new(vec![1, 2, 3], 1_f64, 44100_f64);
        x.sample_right_pad(2);
        assert_eq!(x, Wave::new(vec![1, 2, 3, 0, 0], 1_f64, 44100_f64));
    }

    #[test]
    fn test_milliseconds_right_padding() {
        let mut x: Wave = Wave::new(vec![1], 1_f64, 44100_f64);
        x.milliseconds_right_pad(1000_f64);
        assert_eq!(x.values.first().unwrap(), &1);
        assert_eq!(x.values.len(), 44101);
    }

    #[test]
    fn test_sample_left_padding() {
        let mut x: Wave = Wave::new(vec![1, 2, 3], 1_f64, 44100_f64);
        x.sample_left_pad(2);
        assert_eq!(x, Wave::new(vec![0, 0, 1, 2, 3], 1_f64, 44100_f64));
    }

    #[test]
    fn test_milliseconds_left_padding() {
        let mut x: Wave = Wave::new(vec![1], 1_f64, 44100_f64);
        x.milliseconds_left_pad(1000_f64);
        assert_eq!(x.values.last().unwrap(), &1);
        assert_eq!(x.values.len(), 44101);
    }
}
