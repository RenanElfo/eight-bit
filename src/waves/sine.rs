use std::f64::consts::PI;

use builder_derive_macro::Setters;

use crate::audio::{Audio, AudioBuilder};
use crate::time::has_duration::HasDuration;
use crate::time::has_sampling_frequency::HasSamplingFrequency;
use crate::time::{infer_number_of_samples, samples_to_seconds};
use crate::utils::build::Build;
use crate::waves::traits::has_amplitude::HasAmplitude;
use crate::waves::traits::has_phase::HasPhase;
use crate::waves::traits::has_tone::HasTone;
use crate::{
    impl_has_amplitude, impl_has_duration, impl_has_phase, impl_has_sampling_frequency,
    impl_has_tone,
};

use super::{InvalidWaveForm, InvalidWaveFormKind};

type UpdaterFunction = Option<fn(SineBuilder, usize) -> Sine>;

#[derive(Clone, Debug, PartialEq, Setters)]
pub struct SineBuilder {
    tone: f64,
    amplitude: f64,
    phase_rad: f64,
    duration_ms: f64,
    sampling_frequency: f64,
    updater: UpdaterFunction,
}

impl Default for SineBuilder {
    fn default() -> Self {
        return Self {
            tone: 0.0,
            amplitude: 1.0,
            phase_rad: 0.0,
            duration_ms: 0.0,
            sampling_frequency: 44100_f64,
            updater: None,
        };
    }
}

#[allow(dead_code)]
impl Build for SineBuilder {
    type Output = Sine;
    type Error = InvalidWaveForm;

    fn validate(&self) -> Result<(), Vec<Self::Error>> {
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

    fn finalize(self) -> Result<Self::Output, Self::Error> {
        if let Result::Err(error) = self.validate() {
            return Err(error[0].clone());
        }
        return Ok(Sine {
            tone: self.tone,
            amplitude: self.amplitude,
            phase_rad: self.phase_rad,
            duration_ms: self.duration_ms,
            sampling_frequency: self.sampling_frequency,
            sample_index: 0,
            updater: self.updater,
        });
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sine {
    tone: f64,
    amplitude: f64,
    phase_rad: f64,
    duration_ms: f64,
    sampling_frequency: f64,
    sample_index: usize,
    updater: UpdaterFunction,
}

impl Sine {
    fn sanitize(&mut self) {
        if self.tone.is_infinite() || self.tone.is_nan() {
            self.tone = 0.0;
        }
        if self.amplitude.is_infinite() || self.amplitude.is_nan() {
            self.amplitude = 0.0;
        }
        if self.phase_rad.is_infinite() || self.phase_rad.is_nan() {
            self.phase_rad = 0.0;
        }
        if self.duration_ms < 0.0 || self.duration_ms.is_infinite() || self.duration_ms.is_nan() {
            self.duration_ms = 0.0;
        }
        if self.sampling_frequency < 0.0
            || self.sampling_frequency.is_infinite()
            || self.sampling_frequency.is_nan()
        {
            self.sampling_frequency = 0.0;
        }
    }
}

impl_has_tone!(Sine);
impl_has_amplitude!(Sine);
impl_has_phase!(Sine);
impl_has_duration!(Sine);
impl_has_sampling_frequency!(Sine);

impl Into<SineBuilder> for &mut Sine {
    fn into(self) -> SineBuilder {
        return SineBuilder {
            tone: self.tone,
            amplitude: self.amplitude,
            phase_rad: self.phase_rad,
            duration_ms: self.duration_ms,
            sampling_frequency: self.sampling_frequency,
            updater: self.updater,
        };
    }
}

impl Iterator for Sine {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        self.sanitize();
        let sample_index = self.sample_index;
        if sample_index >= infer_number_of_samples(self) {
            return None;
        }
        let frequency: f64 = self.tone.into();
        let time = samples_to_seconds(self.sampling_frequency, sample_index);
        let sample = self.amplitude * (2.0 * PI * frequency * time + self.phase_rad).sin();
        if let Option::Some(updater_function) = self.updater {
            let builder = Into::<SineBuilder>::into(&mut *self);
            *self = updater_function(builder, self.sample_index);
            self.sanitize();
        }
        self.sample_index = sample_index + 1;
        return Some(sample);
    }
}

impl Into<Audio> for Sine {
    fn into(self) -> Audio {
        let sampling_frequency = self.sampling_frequency;
        let builder = AudioBuilder::new(self.collect(), sampling_frequency);
        return builder.finalize().expect("TODO");
    }
}
