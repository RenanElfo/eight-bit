pub fn duty_cycle(f: f64) -> u8 {
    return (f.clamp(0.0, 1.0) * 255.0).ceil() as u8;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duty_cycle_edge_cases() {
        assert_eq!(duty_cycle(-1.0000000000000002_f64), 0_u8);
        assert_eq!(duty_cycle(1.0000000000000002_f64), 255_u8);
    }

    #[test]
    fn test_duty_cycle() {
        assert_eq!(duty_cycle(0_f64), 0_u8);
        assert_eq!(duty_cycle(0.5_f64), 128_u8);
        assert_eq!(duty_cycle(1_f64), 255_u8);
    }
}
