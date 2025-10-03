use std::f64::consts::PI;

use builder_derive_macro::Setters;

use crate::utils::build::Build;
use crate::audio::{Audio, AudioBuilder, InvalidAudio, traits::ToAudio};
use crate::tone;
use crate::waves::traits::has_tone::HasTone;
use crate::waves::traits::has_amplitude::HasAmplitude;
use crate::waves::traits::has_phase::HasPhase;
use crate::audio::traits::HasSamplingFrequency;
use crate::{impl_has_tone, impl_has_amplitude, impl_has_phase, impl_has_sampling_frequency};

use super::{InvalidWaveForm, InvalidWaveFormKind};

type UpdaterFunction = Option<fn(SineBuilder, usize) -> Sine>;

#[derive(Clone, Debug, PartialEq, Setters)]
pub struct SineBuilder {
    tone: tone::Tone,
    amplitude: f64,
    phase_rad: f64,
    duration_ms: f64,
    sampling_frequency: f64,
    updater: UpdaterFunction,
}

impl Default for SineBuilder {
    fn default() -> Self {
        return Self {
            tone: tone::Tone::default(),
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

#[derive(Clone, Debug, PartialEq)]
pub struct Sine {
    tone: tone::Tone,
    amplitude: f64,
    phase_rad: f64,
    duration_ms: f64,
    sampling_frequency: f64,
    sample_index: usize,
    updater: UpdaterFunction,
}

impl_has_tone!(Sine);
impl_has_amplitude!(Sine);
impl_has_phase!(Sine);
impl_has_sampling_frequency!(Sine);

impl Sine {
    fn number_of_samples(&self) -> usize {
        return Audio::milliseconds_to_samples(self.sampling_frequency, self.duration_ms);
    }
}

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
        let sample_index = self.sample_index;
        if sample_index >= self.number_of_samples() {
            return None;
        }
        let frequency: f64 = self.tone.into();
        let time = Audio::samples_to_seconds(self.sampling_frequency, sample_index);
        let sample = self.amplitude * (2.0 * PI * frequency * time + self.phase_rad).sin();
        if let Option::Some(updater_function) = self.updater {
            let builder = Into::<SineBuilder>::into(&mut *self);
            *self = updater_function(builder, self.sample_index);
        }
        self.sample_index = sample_index + 1;
        return Some(sample);
    }
}

impl ToAudio for Sine {
    fn to_audio(self) -> Result<Audio, InvalidAudio> {
        let sampling_frequency = self.sampling_frequency;
        let builder = AudioBuilder::new(self.collect(), sampling_frequency);
        return builder.finalize();
    }
}
