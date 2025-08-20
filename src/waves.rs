use std::f64::consts::PI;

use builder_derive_macro::{Finalize, Setters};

use crate::audio::{Audio, AudioBuilder, InvalidAudio, ToAudio};
use crate::tone;

#[derive(Clone, Debug, PartialEq)]
pub enum InvalidWaveFormKind {
    NegativeDuration,
    NegativeDutyCycle,
    DutyCycleBiggerThanOne,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InvalidWaveForm {
    kind: InvalidWaveFormKind,
}

#[derive(Clone, Debug, PartialEq, Setters, Finalize)]
pub struct PulseBuilder {
    #[bounds(1, 2, 3)]
    tone: tone::Tone,
    amplitude: f64,
    duration_ms: f64,
    #[bounds(1, ,)]
    rad_phase: f64,
    #[bounds(0.0, 1.0)]
    duty_cycle: f64,
    sampling_frequency: f64,
}

impl Default for PulseBuilder {
    fn default() -> Self {
        return Self {
            tone: tone::Tone::default(),
            amplitude: 1.0,
            duration_ms: 0.0,
            rad_phase: 0.0,
            duty_cycle: 0.5,
            sampling_frequency: 44100_f64,
        };
    }
}

#[allow(dead_code)]
impl PulseBuilder {
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
        if self.duty_cycle < 0.0 {
            possible_errors.push(InvalidWaveForm {
                kind: InvalidWaveFormKind::NegativeDutyCycle,
            });
        }
        if self.duty_cycle > 1.0 {
            possible_errors.push(InvalidWaveForm {
                kind: InvalidWaveFormKind::DutyCycleBiggerThanOne,
            });
        }
        if !possible_errors.is_empty() {
            return Err(possible_errors);
        };
        return Ok(());
    }

    pub fn finalize(self) -> Result<Pulse, InvalidWaveForm> {
        if let Result::Err(error) = self.validate() {
            return Err(error[0].clone());
        }
        return Ok(Pulse {
            tone: self.tone,
            amplitude: self.amplitude,
            duration_ms: self.duration_ms,
            rad_phase: self.rad_phase,
            duty_cycle: self.duty_cycle,
            sampling_frequency: self.sampling_frequency,
        });
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pulse {
    tone: tone::Tone,
    amplitude: f64,
    duration_ms: f64,
    rad_phase: f64,
    duty_cycle: f64,
    sampling_frequency: f64,
}

impl ToAudio for Pulse {
    fn to_audio(&self) -> Result<Audio, InvalidAudio> {
        let frequency: f64 = self.tone.into();
        let period = 1.0 / frequency;
        let number_of_samples =
            Audio::milliseconds_to_samples(self.sampling_frequency, self.duration_ms);
        let indices = 0..number_of_samples;
        let samples: Vec<f64> = indices
            .into_iter()
            .map(|sample_index| {
                let time = Audio::samples_to_seconds(self.sampling_frequency, sample_index);
                let time_in_period = (time + self.rad_phase * period / (2.0 * PI)) % period;
                if time_in_period <= self.duty_cycle * period {
                    return self.amplitude;
                }
                return -self.amplitude;
            })
            .collect();
        let builder = AudioBuilder::new(samples, self.sampling_frequency);
        return builder.finalize();
    }
}

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
    fn to_audio(&self) -> Result<Audio, InvalidAudio> {
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

#[derive(Clone, Debug, PartialEq, Setters)]
pub struct TriangleBuilder {
    tone: tone::Tone,
    amplitude: f64,
    duration_ms: f64,
    rad_phase: f64,
    sampling_frequency: f64,
}

impl Default for TriangleBuilder {
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
            duration_ms: self.duration_ms,
            rad_phase: self.rad_phase,
            sampling_frequency: self.sampling_frequency,
        });
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Triangle {
    tone: tone::Tone,
    amplitude: f64,
    duration_ms: f64,
    rad_phase: f64,
    sampling_frequency: f64,
}

impl ToAudio for Triangle {
    fn to_audio(&self) -> Result<Audio, InvalidAudio> {
        let frequency: f64 = self.tone.into();
        let period: f64 = 1.0 / frequency;
        let number_of_samples =
            Audio::milliseconds_to_samples(self.sampling_frequency, self.duration_ms);
        let indices = 0..number_of_samples;
        let samples: Vec<f64> = indices
            .into_iter()
            .map(|sample_index| {
                let time = Audio::samples_to_seconds(self.sampling_frequency, sample_index);
                let time_with_phase = time + self.rad_phase * period / (2.0 * PI);
                let modulus = |x, m| ((x % m) + m) % m;
                return 4.0
                    * self.amplitude
                    * frequency
                    * f64::abs(modulus(time_with_phase - period / 4.0, period) - period / 2.0)
                    - self.amplitude;
            })
            .collect();
        let builder = AudioBuilder::new(samples, self.sampling_frequency);
        return builder.finalize();
    }
}

#[derive(Clone, Debug, PartialEq, Setters)]
pub struct SawtoothBuilder {
    tone: tone::Tone,
    amplitude: f64,
    duration_ms: f64,
    rad_phase: f64,
    sampling_frequency: f64,
}

impl Default for SawtoothBuilder {
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
impl SawtoothBuilder {
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

    pub fn finalize(self) -> Result<Sawtooth, InvalidWaveForm> {
        if let Result::Err(error) = self.validate() {
            return Err(error[0].clone());
        }
        return Ok(Sawtooth {
            tone: self.tone,
            amplitude: self.amplitude,
            duration_ms: self.duration_ms,
            rad_phase: self.rad_phase,
            sampling_frequency: self.sampling_frequency,
        });
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Sawtooth {
    tone: tone::Tone,
    amplitude: f64,
    duration_ms: f64,
    rad_phase: f64,
    sampling_frequency: f64,
}

impl ToAudio for Sawtooth {
    fn to_audio(&self) -> Result<Audio, InvalidAudio> {
        let frequency: f64 = self.tone.into();
        let period: f64 = 1.0 / frequency;
        let number_of_samples =
            Audio::milliseconds_to_samples(self.sampling_frequency, self.duration_ms);
        let indices = 0..number_of_samples;
        let samples: Vec<f64> = indices
            .into_iter()
            .map(|sample_index| {
                let time = Audio::samples_to_seconds(self.sampling_frequency, sample_index);
                let time_with_phase = time + self.rad_phase * period / (2.0 * PI);
                let modulus = |x, m| ((x % m) + m) % m;
                return 2.0
                    * self.amplitude
                    * frequency
                    * modulus(time_with_phase + period / 2.0, period)
                    - self.amplitude;
            })
            .collect();
        let builder = AudioBuilder::new(samples, self.sampling_frequency);
        return builder.finalize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file() {}

    #[test]
    fn test_to_audio() {
        let x: Pulse = PulseBuilder::default()
            .with_duration_ms(1000.0)
            .finalize()
            .unwrap();
        let audio: Audio = x.to_audio().unwrap();
        assert_eq!(audio.sample_length(), 44100);
    }
}
