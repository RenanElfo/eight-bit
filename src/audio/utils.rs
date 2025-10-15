use super::Audio;

use crate::time::has_sampling_frequency::HasSamplingFrequency;
use crate::time::samples_to_milliseconds;

#[allow(dead_code)]
impl Audio {
    pub fn get_samples(self) -> Vec<f64> {
        return self.samples;
    }

    pub fn sample_length(&self) -> usize {
        return self.samples.len();
    }

    pub fn milliseconds_length(&self) -> f64 {
        let sampling_frequency = self.get_sampling_frequency();
        return samples_to_milliseconds(sampling_frequency, self.sample_length());
    }

    pub fn samples_as_vec_16(self) -> Vec<i16> {
        let samples = self.get_samples();
        let max = samples
            .iter()
            .map(|sample| sample.abs())
            .reduce(f64::max)
            .unwrap_or(0.0);
        let new_vec: Vec<i16> = samples
            .into_iter()
            .map(|sample| (i16::MAX as f64 * sample / max) as i16)
            .collect();
        return new_vec;
    }

    pub fn write_wav(self) {
        let sample_rate = self.get_sampling_frequency() as u32;
        println!("{:?}", sample_rate);
        let vec = self.samples_as_vec_16();
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create("test.wav", spec).unwrap();

        for sample in vec.into_iter() {
            writer.write_sample(sample).unwrap();
        }
        writer.finalize().unwrap();
    }
}
