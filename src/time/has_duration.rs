// use super::has_sampling_frequency::HasSamplingFrequency;
// use crate::audio::Audio;

#[allow(dead_code)]
pub trait HasDuration {
    fn get_duration_ms(&self) -> f64;

    fn set_duration_ms(&mut self, duration_ms: f64);

    fn get_duration_sec(&self) -> f64 {
        self.get_duration_ms() / 1000.0
    }

    fn set_duration_sec(&mut self, duration_sec: f64) {
        self.set_duration_ms(duration_sec * 1000.0);
    }
    //
    // fn get_number_of_samples(&self) -> usize
    // where
    //     Self: HasSamplingFrequency,
    // {
    //     Audio::milliseconds_to_samples(self.get_sampling_frequency(), self.get_duration_ms())
    // }
}

#[macro_export]
macro_rules! impl_has_duration {
    ($name: ty) => {
        impl HasDuration for $name {
            fn get_duration_ms(&self) -> f64 {
                self.duration_ms
            }

            fn set_duration_ms(&mut self, duration_ms: f64) {
                self.duration_ms = duration_ms;
            }
        }
    };
}
