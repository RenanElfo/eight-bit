use std::time::Duration;

pub struct SamplingFrequency {
    period: Duration,
}

impl SamplingFrequency {
    fn new() -> SamplingFrequency {
        todo!()
    }

    pub fn from_period(period: Duration) -> SamplingFrequency {
        return SamplingFrequency { period };
    }
}
