use std::f64::consts::PI;

use builder_derive_macro::{Finalize, Setters};

use crate::audio::{Audio, AudioBuilder, InvalidAudio, ToAudio};
use crate::tone;

use super::{InvalidWaveForm, InvalidWaveFormKind};

#[derive(Clone, Debug, PartialEq, Setters, Finalize)]
pub struct PulseBuilder {
    // #[bounds(1, 2, 3)]
    tone: tone::Tone,
    amplitude: f64,
    duration_ms: f64,
    // #[bounds(1, ,)]
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
    fn to_audio(self) -> Result<Audio, InvalidAudio> {
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
