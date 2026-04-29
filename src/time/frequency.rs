use std::time::Duration;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Frequency {
    period: Duration,
}

impl Frequency {
    pub fn new(period: Duration) -> Frequency {
        return Frequency { period };
    }

    pub fn from_hertz_f64(frequency: f64) -> Frequency {
        return Frequency::from_period(Duration::from_secs_f64(1.0 / frequency));
    }

    pub fn from_kilo_hertz_f64(frequency: f64) -> Frequency {
        return Frequency::from_period(Duration::from_secs_f64(0.001 / frequency));
    }
}

impl Frequency {
    pub fn as_hertz_f64(&self) -> f64 {
        return 1.0 / self.period.as_secs_f64();
    }

    pub fn as_kilo_hertz_f64(&self) -> f64 {
        return 0.001 / self.period.as_secs_f64();
    }
}
