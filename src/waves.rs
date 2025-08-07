use std::f64::consts::PI;

use builder_derive_macro::Setters;

use crate::audio;
use crate::tone;

#[allow(dead_code)]
pub trait ToAudio {
    fn to_audio(&self) -> Result<audio::Audio, audio::InvalidAudio>;
}

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

#[derive(Clone, Debug, PartialEq, Setters)]
pub struct PulseBuilder {
    tone: tone::Tone,
    amplitude: f64,
    duration_ms: f64,
    rad_phase: f64,
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
    pub fn new(
        tone: tone::Tone,
        amplitude: f64,
        duration_ms: f64,
        rad_phase: f64,
        duty_cycle: f64,
        sampling_frequency: f64,
    ) -> Self {
        return Self {
            tone,
            amplitude,
            duration_ms,
            rad_phase,
            duty_cycle,
            sampling_frequency,
        };
    }

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
    fn to_audio(&self) -> Result<audio::Audio, audio::InvalidAudio> {
        let frequency: f64 = self.tone.into();
        let period = 1.0 / frequency;
        let number_of_samples =
            audio::Audio::milliseconds_to_samples(self.sampling_frequency, self.duration_ms);
        let indices = 0..number_of_samples;
        let samples: Vec<f64> = indices
            .into_iter()
            .map(|sample_index| {
                let time_ms =
                    audio::Audio::samples_to_milliseconds(self.sampling_frequency, sample_index);
                let time = time_ms / 1000.0;
                let time_in_period = (time + self.rad_phase * period / (2.0 * PI)) % period;
                if time_in_period <= self.duty_cycle * period {
                    return self.amplitude;
                }
                return -self.amplitude;
            })
            .collect();
        let builder = audio::AudioBuilder::new(samples, self.sampling_frequency);
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
    pub fn new(
        tone: tone::Tone,
        amplitude: f64,
        duration_ms: f64,
        phase: f64,
        sampling_frequency: f64,
    ) -> Self {
        return Self {
            tone,
            amplitude,
            duration_ms,
            rad_phase: phase,
            sampling_frequency,
        };
    }

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
    fn to_audio(&self) -> Result<audio::Audio, audio::InvalidAudio> {
        let frequency: f64 = self.tone.into();
        let number_of_samples =
            audio::Audio::milliseconds_to_samples(self.sampling_frequency, self.duration_ms);
        let indices = 0..number_of_samples;
        let samples: Vec<f64> = indices
            .into_iter()
            .map(|sample_index| {
                let time_ms =
                    audio::Audio::samples_to_milliseconds(self.sampling_frequency, sample_index);
                let time = time_ms / 1000.0;
                return self.amplitude * (2.0 * PI * frequency * time + self.rad_phase).sin();
            })
            .collect();
        let builder = audio::AudioBuilder::new(samples, self.sampling_frequency);
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
        let audio: audio::Audio = x.to_audio(44100_f64).finalize().unwrap();
        assert_eq!(audio.sample_length(), 44100);
    }
}
