use std::time::Duration;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SamplingFrequency {
    period: Duration,
}

impl SamplingFrequency {
    pub fn new(sampling_frequency: f64) -> SamplingFrequency {
        return SamplingFrequency::from_period(Duration::from_secs(1).div_f64(sampling_frequency));
    }

    pub fn from_period(period: Duration) -> SamplingFrequency {
        return SamplingFrequency { period };
    }
}
