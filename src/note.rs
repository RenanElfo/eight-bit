use std::default::Default;

const A_INDEX: usize = 53;
const A_FREQUENCY: f64 = 440_f64;
const SEMI_TONE_FACTOR: f64 = f64::from_bits(4607450216769616227);
const NUMBER_OF_AVAILABLE_NOTES: usize = 120;
pub const AVAILABLE_NOTES: [f64; NUMBER_OF_AVAILABLE_NOTES] = generate_array();

const fn generate_array() -> [f64; NUMBER_OF_AVAILABLE_NOTES] {
    let mut array = [0_f64; NUMBER_OF_AVAILABLE_NOTES];
    array[A_INDEX] = A_FREQUENCY;
    let mut index: usize = A_INDEX;
    while index > 0 {
        array[index - 1] = array[index] / SEMI_TONE_FACTOR;
        index -= 1;
    }
    index = A_INDEX;
    while index < NUMBER_OF_AVAILABLE_NOTES - 1 {
        array[index + 1] = array[index] * SEMI_TONE_FACTOR;
        index += 1;
    }
    return array;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Pitch {
    Frequency(f64),
    Key(u8),
}

impl Default for Pitch {
    fn default() -> Self {
        return Pitch::Frequency(440_f64);
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Note {
    pitch: Pitch,
    duration: f64,
}

impl Note {
    pub fn with_pitch(mut self, pitch: Pitch) -> Self {
        self.pitch = pitch;
        return self;
    }

    pub fn with_duration(mut self, duration: f64) -> Self {
        self.duration = duration;
        return self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file() {}

    #[test]
    fn test_enum() {
        let x: Pitch = Pitch::Frequency(A_FREQUENCY);
        let mut y: f64 = 0_f64;
        if let Pitch::Frequency(value) = x {
            y = value.clone();
            assert_eq!(value, A_FREQUENCY);
        }
        assert_eq!(y, A_FREQUENCY);
    }
}
