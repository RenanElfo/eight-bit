#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_enum() {
        let x: Tone = AvailableTones::try_from(A_FREQUENCY)
            .expect("A_FREQUENCY should have been a valid frequency")
            .as_pitch();
        if let AvailableTones::Pitch(value) = x {
            assert_eq!(value, A_FREQUENCY);
        }
        assert_eq!(
            x.as_note()
                .expect("A_FREQUENCY should have been a valid frequency"),
            AvailableTones::default()
        );
    }

    #[test]
    fn test_conversions() {
        let x = AvailableTones::try_from(A_FREQUENCY).unwrap();
        assert_eq!(Into::<f64>::into(x), A_FREQUENCY);
    }
}
