#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_builder_functions() {
        let sampling_frequency = 8192_f64;
        let x: AudioBuilder = AudioBuilder::default()
            .with_length(3)
            .with_sampling_frequency(8192_f64);
        assert_eq!(x.samples, vec![0.0, 0.0, 0.0]);
        assert_eq!(x.sampling_frequency, sampling_frequency);
    }

    #[test]
    fn test_concatenation() {
        let x: Audio = AudioBuilder::new(vec![1.0, 2.0, 3.0], 44100_f64)
            .finalize()
            .unwrap();
        let y: Audio = AudioBuilder::new(vec![4.0, 5.0, 6.0], 44100_f64)
            .finalize()
            .unwrap();
        let z = x.clone().merge(y.clone()).unwrap();
        assert_eq!(
            z,
            AudioBuilder::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 44100_f64)
                .finalize()
                .unwrap()
        );
        let w = x - y;
        assert_eq!(z, w);
    }

    #[test]
    fn test_overlapping() {
        let x: Audio = AudioBuilder::new(vec![1.0], 44100_f64).finalize().unwrap();
        let y: Audio = AudioBuilder::new(vec![0.5], 44100_f64).finalize().unwrap();
        let z: Audio = x.clone().overlap(y.clone()).unwrap();
        assert_eq!(
            z,
            AudioBuilder::new(vec![1.5], 44100_f64).finalize().unwrap()
        );
        let w = x / y;
        assert_eq!(z, w.unwrap());
    }

    #[test]
    fn test_sample_right_padding() {
        let mut x: Audio = AudioBuilder::new(vec![1.0, 2.0, 3.0], 44100_f64)
            .finalize()
            .unwrap();
        x.sample_right_pad(2);
        assert_eq!(
            x,
            AudioBuilder::new(vec![1.0, 2.0, 3.0, 0.0, 0.0], 44100_f64)
                .finalize()
                .unwrap()
        );
    }

    #[test]
    fn test_milliseconds_right_padding() {
        let mut x: Audio = AudioBuilder::new(vec![1.0], 44100_f64).finalize().unwrap();
        x.milliseconds_right_pad(1000_f64);
        assert_eq!(x.samples.first().unwrap(), &1.0);
        assert_eq!(x.samples.len(), 44101);
    }

    #[test]
    fn test_sample_left_padding() {
        let mut x: Audio = AudioBuilder::new(vec![1.0, 2.0, 3.0], 44100_f64)
            .finalize()
            .unwrap();
        x.sample_left_pad(2);
        assert_eq!(
            x,
            AudioBuilder::new(vec![0.0, 0.0, 1.0, 2.0, 3.0], 44100_f64)
                .finalize()
                .unwrap()
        );
    }

    #[test]
    fn test_milliseconds_left_padding() {
        let mut x: Audio = AudioBuilder::new(vec![1.0], 44100_f64).finalize().unwrap();
        x.milliseconds_left_pad(1000_f64);
        assert_eq!(x.samples.last().unwrap(), &1.0);
        assert_eq!(x.samples.len(), 44101);
    }
}
