use crate::time::has_sampling_frequency::HasSamplingFrequency;
use crate::time::samples_to_milliseconds;

use super::traits::FilterAudio;
use super::Audio;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BitCruncher(pub u32);

impl FilterAudio for BitCruncher {
    fn filter(self, audio: Audio) -> Audio {
        let sampling_frequency = audio.get_sampling_frequency();
        let samples = audio.get_samples();
        let amplitude = samples
            .iter()
            .map(|sample| sample.abs())
            .reduce(f64::max)
            .unwrap_or(0.0);
        let samples = samples
            .into_iter()
            .map(|sample| {
                let shift = if self.0 <= u64::BITS {
                    u64::BITS - self.0
                } else {
                    0
                };
                let crunched = ((sample / amplitude) * u64::MAX as f64) as u64 >> shift << shift;
                crunched as f64 * amplitude
            })
            .collect();
        let mut filtered = Audio {
            sampling_frequency: None,
            samples,
        };
        filtered.set_sampling_frequency(sampling_frequency);
        filtered
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SimpleDownsampler(pub u16);

impl FilterAudio for SimpleDownsampler {
    fn filter(self, audio: Audio) -> Audio {
        let factor = if self.0 != 0 { self.0 } else { 1 };
        let sampling_frequency = audio.get_sampling_frequency();
        let samples: Vec<_> = audio
            .get_samples()
            .into_iter()
            .step_by(factor as usize)
            .collect();
        let mut filtered = Audio {
            sampling_frequency: None,
            samples,
        };
        filtered.set_sampling_frequency(sampling_frequency / factor as f64);
        filtered
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SimpleUpsampler(pub u16);

impl FilterAudio for SimpleUpsampler {
    fn filter(self, audio: Audio) -> Audio {
        let factor = if self.0 != 0 { self.0 } else { 1 };
        let sampling_frequency = audio.get_sampling_frequency();
        let samples: Vec<_> = audio
            .get_samples()
            .into_iter()
            .flat_map(|sample| {
                std::iter::once(sample).chain(std::iter::repeat(0.0).take((factor - 1) as usize))
            })
            .collect();
        let mut filtered = Audio {
            sampling_frequency: None,
            samples,
        };
        filtered.set_sampling_frequency(sampling_frequency * factor as f64);
        filtered
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Decay(pub f64);

impl FilterAudio for Decay {
    fn filter(self, audio: Audio) -> Audio {
        let sampling_frequency = audio.get_sampling_frequency();
        let samples: Vec<_> = audio
            .get_samples()
            .into_iter()
            .enumerate()
            .map(|(index, sample)| {
                let time = samples_to_milliseconds(sampling_frequency, index);
                let new_sample = sample * f64::exp(f64::ln(0.5) / self.0 * time);
                new_sample
            })
            .collect();
        let mut filtered = Audio {
            sampling_frequency: None,
            samples,
        };
        filtered.set_sampling_frequency(sampling_frequency);
        filtered
    }
}
