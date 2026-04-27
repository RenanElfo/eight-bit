pub trait HasDuration {
    fn get_duration_ms(&self) -> f64;

    fn set_duration_ms(&mut self, duration_ms: f64);

    fn get_duration_sec(&self) -> f64 {
        self.get_duration_ms() / 1000.0
    }
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
