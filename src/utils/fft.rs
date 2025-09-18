use rustfft::{num_complex::Complex, FftPlanner};

pub fn rfft_freq_bins(length: usize, sampling_frequency: f64) -> Vec<f64> {
    (0..=length / 2)
        .map(|sample_index| sample_index as f64 * sampling_frequency / length as f64)
        .collect()
}

pub fn rfft(signal: &Vec<f64>) -> Vec<Complex<f64>> {
    let length: usize = signal.len();
    let mut signal: Vec<Complex<f64>> = signal
        .iter()
        .map(|sample| Complex {
            re: *sample,
            im: 0.0,
        })
        .collect();
    let mut fft_planner = FftPlanner::new();
    let fft = fft_planner.plan_fft_forward(length);
    fft.process(&mut signal);
    signal[0..=length / 2].to_vec()
}

pub fn irfft(signal_rfft: Vec<Complex<f64>>, length: usize) -> Vec<f64> {
    let hermitian_slice = 1..=((length - 1) / 2);
    let hermitian_symmetry: Vec<Complex<f64>> = signal_rfft[hermitian_slice]
        .iter()
        .map(|value| {
            return value.conj();
        })
        .rev()
        .collect();
    let mut signal_fft: Vec<Complex<f64>> = [signal_rfft, hermitian_symmetry].concat();
    let mut fft_planner = FftPlanner::new();
    let fft = fft_planner.plan_fft_inverse(length);
    assert_eq!(length, signal_fft.len());
    fft.process(&mut signal_fft);
    signal_fft
        .iter()
        .map(|sample| sample.re / length as f64)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fft() {
        let epsilon = 0.0001;
        let samples: Vec<f64> = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5];
        let length = samples.len();
        let samples_rfft = rfft(&samples);
        let samples_irfft = irfft(samples_rfft.clone(), length);
        assert_eq!(length, samples_irfft.len());
        assert!(samples_rfft.clone().first().unwrap().im.abs() < epsilon);
        assert!(samples_rfft.clone().last().unwrap().im.abs() < epsilon);
        let mut sub: Vec<f64> = vec![0.0; length];
        for i in 0..length {
            sub[i] = (samples[i] - samples_irfft[i]).abs();
        }
        assert!(sub.into_iter().all(|diff| diff < epsilon));
        let samples: Vec<f64> = vec![0.0, 0.1, 0.2, 0.3, 0.4];
        let length = samples.len();
        let samples_rfft = rfft(&samples);
        let samples_irfft = irfft(samples_rfft.clone(), length);
        assert_eq!(length, samples_irfft.len());
        let mut sub: Vec<f64> = vec![0.0; length];
        for i in 0..length {
            sub[i] = (samples[i] - samples_irfft[i]).abs();
        }
        assert!(sub.into_iter().all(|diff| diff < epsilon));
    }
}
