#[allow(dead_code)]
pub trait HasAmplitude {
    fn get_amplitude(&self) -> f64;

    fn set_amplitude(&mut self, amplitude: f64);
}

#[macro_export]
macro_rules! impl_has_amplitude {
    ($name: ty) => {
        impl HasAmplitude for $name {
            fn get_amplitude(&self) -> f64 {
                self.amplitude
            }

            fn set_amplitude(&mut self, amplitude: f64) {
                self.amplitude = amplitude;
            }
        }
    };
}
