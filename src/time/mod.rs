pub mod has_duration;
use has_duration::HasDuration;
pub mod has_sampling_frequency;
use has_sampling_frequency::HasSamplingFrequency;

#[allow(dead_code)]
pub fn infer_number_of_samples<T>(variable: &T) -> usize
where
    T: HasDuration + HasSamplingFrequency,
{
    return (variable.get_duration_sec() * variable.get_sampling_frequency()) as usize;
}

pub fn samples_to_milliseconds(sampling_frequency: f64, amount: usize) -> f64 {
    if amount == 0 || sampling_frequency <= 0.0 || sampling_frequency.is_nan() {
        return 0.0;
    }
    return (amount as f64) * 1000_f64 / sampling_frequency;
}

pub fn samples_to_seconds(sampling_frequency: f64, amount: usize) -> f64 {
    if amount == 0 || sampling_frequency <= 0.0 || sampling_frequency.is_nan() {
        return 0.0;
    }
    return (amount as f64) / sampling_frequency;
}

pub fn milliseconds_to_samples(sampling_frequency: f64, time_interval: f64) -> usize {
    return ((time_interval / 1000_f64) * sampling_frequency) as usize;
}
