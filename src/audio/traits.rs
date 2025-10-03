use super::{Audio, InvalidAudio};

#[allow(dead_code)]
pub trait ToAudio {
    fn to_audio(self) -> Result<Audio, InvalidAudio>;
}

#[allow(dead_code)]
pub trait FilterAudio {
    fn filter(self, audio: Audio) -> Audio;
}

#[allow(dead_code)]
pub trait HasSamplingFrequency {
    fn get_sampling_frequency(&self) -> f64;

    fn set_sampling_frequency(&mut self, tone: f64);
}

#[macro_export]
macro_rules! impl_has_sampling_frequency {
    ($name: ty) => {
        impl HasSamplingFrequency for $name {
            fn get_sampling_frequency(&self) -> f64 {
                self.sampling_frequency
            }

            fn set_sampling_frequency(&mut self, sampling_frequency: f64) {
                self.sampling_frequency = sampling_frequency;
            }
        }
    };
}
