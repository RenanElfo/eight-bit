use super::Audio;

#[allow(dead_code)]
impl Audio {
    pub fn sample_length(&self) -> usize {
        return self.samples.len();
    }

    pub fn milliseconds_length(&self) -> f64 {
        return Audio::samples_to_milliseconds(self.sampling_frequency, self.sample_length());
    }

    pub fn samples_to_milliseconds(sampling_frequency: f64, ammount: usize) -> f64 {
        return (ammount as f64) * 1000_f64 / sampling_frequency;
    }

    pub fn samples_to_seconds(sampling_frequency: f64, ammount: usize) -> f64 {
        return (ammount as f64) / sampling_frequency;
    }

    pub fn milliseconds_to_samples(sampling_frequency: f64, time_interval: f64) -> usize {
        return ((time_interval / 1000_f64) * sampling_frequency) as usize;
    }

    pub fn samples_as_vec_16(samples: Vec<f64>) -> Vec<i16> {
        let max = samples
            .clone()
            .into_iter()
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
        let sample_rate = self.sampling_frequency as u32;
        let vec = Audio::samples_as_vec_16(self.get_samples());
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

