use std::f64::consts::PI;

use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};
use rustfft::{num_complex::Complex, FftPlanner};

use builder_derive_macro::{Finalize, Setters};

use crate::audio::{Audio, AudioBuilder, InvalidAudio, ToAudio};
use crate::utils::fft::{rfft, irfft, rfft_freq_bins};

use super::{InvalidWaveForm, InvalidWaveFormKind};

#[derive(Clone, Debug, Default, PartialEq)]
pub enum NoiseVariant {
    Violet,
    Blue,
    #[default]
    White,
    Pink,
    Brown,
}

#[derive(Clone, Debug, PartialEq, Setters, Finalize)]
pub struct NoiseBuilder {
    // #[bounds(1, 2, 3)]
    amplitude: f64,
    duration_ms: f64,
    // #[bounds(1, ,)]
    seed: u64,
    variant: NoiseVariant,
    #[bounds(0.0, 1.0)]
    sampling_frequency: f64,
}

impl Default for NoiseBuilder {
    fn default() -> Self {
        return Self {
            amplitude: 1.0,
            duration_ms: 0.0,
            seed: 1,
            variant: NoiseVariant::default(),
            sampling_frequency: 44100_f64,
        };
    }
}

#[allow(dead_code)]
impl NoiseBuilder {
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

    pub fn finalize(self) -> Result<Noise, InvalidWaveForm> {
        if let Result::Err(error) = self.validate() {
            return Err(error[0].clone());
        }
        return Ok(Noise {
            amplitude: self.amplitude,
            duration_ms: self.duration_ms,
            seed: self.seed,
            variant: self.variant,
            sampling_frequency: self.sampling_frequency,
        });
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Noise {
    amplitude: f64,
    duration_ms: f64,
    seed: u64,
    variant: NoiseVariant,
    sampling_frequency: f64,
}

fn get_noise(
    sampling_frequency: f64,
    number_of_samples: usize,
    normal_samples: Vec<f64>,
    noise_closure: fn(f64) -> f64,
) -> Result<Audio, InvalidAudio> {
    let normal_samples_rfft = rfft(&normal_samples);
    let spectral_density =
        generate_power_spectral_density(number_of_samples, sampling_frequency, noise_closure);
    assert_eq!(normal_samples_rfft.len(), spectral_density.len());
    let rfft_len = normal_samples_rfft.len();
    let mut noise_samples_rfft: Vec<Complex<f64>> = vec![Complex { re: 0.0, im: 0.0 }; rfft_len];
    for i in 0..rfft_len {
        noise_samples_rfft[i] = normal_samples_rfft[i] * spectral_density[i];
    }
    let noise_samples = irfft(noise_samples_rfft, number_of_samples);
    let builder = AudioBuilder::new(noise_samples, sampling_frequency);
    builder.finalize()
}

impl ToAudio for Noise {
    fn to_audio(self) -> Result<Audio, InvalidAudio> {
        let mut random_number_generator = SmallRng::seed_from_u64(self.seed);
        let number_of_samples =
            Audio::milliseconds_to_samples(self.sampling_frequency, self.duration_ms);
        let indices = 0..number_of_samples;
        let normal_samples: Vec<f64> = indices
            .into_iter()
            .map(|_sample_index| {
                let uniform_sample_1 = random_number_generator.next_u32() as f64 / u32::MAX as f64;
                let uniform_sample_2 = random_number_generator.next_u32() as f64 / u32::MAX as f64;
                let normal_sample = self.amplitude
                    * f64::sqrt(-2.0 * f64::ln(uniform_sample_1))
                    * f64::cos(2.0 * PI * uniform_sample_2);
                return normal_sample;
            })
            .collect();
        return match self.variant {
            NoiseVariant::Violet => {
                let blue_noise_function = |freq: f64| freq;
                get_noise(
                    self.sampling_frequency,
                    number_of_samples,
                    normal_samples,
                    blue_noise_function,
                )
            }
            NoiseVariant::Blue => {
                let blue_noise_function = |freq: f64| f64::sqrt(freq);
                get_noise(
                    self.sampling_frequency,
                    number_of_samples,
                    normal_samples,
                    blue_noise_function,
                )
            }
            NoiseVariant::White => {
                let builder = AudioBuilder::new(normal_samples, self.sampling_frequency);
                builder.finalize()
            }
            NoiseVariant::Pink => {
                let pink_noise_function = |freq: f64| {
                    if freq != 0.0 {
                        1.0 / f64::sqrt(freq)
                    } else {
                        0.0
                    }
                };
                get_noise(
                    self.sampling_frequency,
                    number_of_samples,
                    normal_samples,
                    pink_noise_function,
                )
            }
            NoiseVariant::Brown => {
                let brown_noise_function = |freq: f64| if freq != 0.0 { 1.0 / freq } else { 0.0 };
                get_noise(
                    self.sampling_frequency,
                    number_of_samples,
                    normal_samples,
                    brown_noise_function,
                )
            }
        };
    }
}

fn generate_power_spectral_density<F: Fn(f64) -> f64>(
    length: usize,
    sampling_frequency: f64,
    power_spectral_density_closure: F,
) -> Vec<f64> {
    let frequency_bins = rfft_freq_bins(length, sampling_frequency);
    let spectrum_density: Vec<f64> = frequency_bins
        .into_iter()
        .map(power_spectral_density_closure)
        .collect();
    let power = spectrum_density
        .iter()
        .map(|density| density * density)
        .reduce(|accumulator, element| accumulator + element / spectrum_density.len() as f64)
        .unwrap_or(1.0)
        .sqrt();
    let normalized_spectrum_density: Vec<f64> = spectrum_density
        .into_iter()
        .map(|density| density / power)
        .collect();
    return normalized_spectrum_density;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fft() {
        let epsilon = 0.0001;
        let samples: Vec<f64> = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5];
        let length = samples.len();
        let samples_rfft = rfft(&samples);
        let samples_irfft = irfft(samples_rfft.clone(), length);
        assert_eq!(length, samples_irfft.len());
        assert!(samples_rfft.clone().first().unwrap().im.abs() < epsilon);
        assert!(samples_rfft.clone().last().unwrap().im.abs() < epsilon);
        let mut sub: Vec<f64> = vec![0.0; length];
        for i in 0..length {
            sub[i] = (samples[i] - samples_irfft[i]).abs();
        }
        assert!(sub.into_iter().all(|diff| diff < epsilon));
        let samples: Vec<f64> = vec![0.0, 0.1, 0.2, 0.3, 0.4];
        let length = samples.len();
        let samples_rfft = rfft(&samples);
        let samples_irfft = irfft(samples_rfft.clone(), length);
        assert_eq!(length, samples_irfft.len());
        let mut sub: Vec<f64> = vec![0.0; length];
        for i in 0..length {
            sub[i] = (samples[i] - samples_irfft[i]).abs();
        }
        assert!(sub.into_iter().all(|diff| diff < epsilon));
    }
}
