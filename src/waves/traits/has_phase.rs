#[allow(dead_code)]
pub trait HasPhase {
    fn get_phase_rad(&self) -> f64;

    fn set_phase_rad(&mut self, phase_rad: f64);

    fn get_phase_deg(&self) -> f64 {
        180.0 * self.get_phase_rad() / std::f64::consts::PI
    }

    fn set_phase_deg(&mut self, phase_deg: f64) {
        self.set_phase_rad(phase_deg * std::f64::consts::PI / 180.0);
    }
}

#[macro_export]
macro_rules! impl_has_phase {
    ($name: ty) => {
        impl HasPhase for $name {
            fn get_phase_rad(&self) -> f64 {
                self.phase_rad
            }

            fn set_phase_rad(&mut self, phase_rad: f64) {
                self.phase_rad = phase_rad;
            }
        }
    };
}
