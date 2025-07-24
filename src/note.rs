use std::convert::{TryFrom, TryInto};
use std::default::Default;

use crate::standard_notes::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AvailableNote<const MAX_SIZE: usize> {
    index: usize,
}

impl TryFrom<usize> for AvailableNote<NUMBER_OF_AVAILABLE_NOTES> {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value >= NUMBER_OF_AVAILABLE_NOTES {
            return Err("Passed usize value is larger than maximum allowed.");
        }
        return Ok(AvailableNote { index: value });
    }
}

impl Into<f64> for AvailableNote<NUMBER_OF_AVAILABLE_NOTES> {
    fn into(self: Self) -> f64 {
        return AVAILABLE_NOTES[self.index];
    }
}

impl Into<Frequency> for AvailableNote<NUMBER_OF_AVAILABLE_NOTES> {
    fn into(self: Self) -> Frequency {
        let frequency: f64 = self.into();
        return Frequency::try_from(frequency).unwrap();
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frequency {
    value: f64,
}

impl TryFrom<f64> for Frequency {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value < 0_f64 {
            return Err("Frequency value must be non-negative.");
        }
        return Ok(Frequency { value });
    }
}

impl Into<f64> for Frequency {
    fn into(self: Self) -> f64 {
        return self.value;
    }
}

impl TryInto<AvailableNote<NUMBER_OF_AVAILABLE_NOTES>> for Frequency {
    type Error = &'static str;

    fn try_into(self: Self) -> Result<AvailableNote<NUMBER_OF_AVAILABLE_NOTES>, Self::Error> {
        let possible_notes_indices = AVAILABLE_NOTES
            .into_iter()
            .enumerate()
            .filter(|(_index, note)| {
                let distance = (note - self.value).abs();
                return distance < LOWEST_NOTE;
            })
            .map(|(index, _note)| index)
            .nth(0);
        match possible_notes_indices {
            Some(index) => return AvailableNote::try_from(index),
            None => return Err("Hello World"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tone<const MAX_SIZE: usize> {
    Pitch(Frequency),
    Note(AvailableNote<MAX_SIZE>),
}

impl Default for Tone<NUMBER_OF_AVAILABLE_NOTES> {
    fn default() -> Self {
        return Tone::Pitch(Frequency::try_from(440_f64).unwrap());
    }
}

// #[derive(Clone, Copy, Debug, Default, PartialEq)]
// pub struct Sound {
//     pitch: Tone<NUMBER_OF_AVAILABLE_NOTES>,
//     duration: f64,
// }
//
// impl Sound {
//     pub fn with_pitch(mut self, pitch: Tone<NUMBER_OF_AVAILABLE_NOTES>) -> Self {
//         self.pitch = pitch;
//         return self;
//     }
//
//     pub fn with_duration(mut self, duration: f64) -> Self {
//         self.duration = duration;
//         return self;
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file() {}

    #[test]
    fn test_enum() {
        let x: Tone<NUMBER_OF_AVAILABLE_NOTES> =
            Tone::Pitch(Frequency::try_from(A_FREQUENCY).unwrap());
        let mut y: f64 = 0_f64;
        if let Tone::Pitch(value) = x {
            y = value.into();
            assert_eq!(value, Frequency::try_from(A_FREQUENCY).unwrap());
        }
        assert_eq!(y, A_FREQUENCY);
        assert_eq!(x, Tone::default());
    }

    #[test]
    fn test_available_note_conversions() {
        let x: AvailableNote<NUMBER_OF_AVAILABLE_NOTES> =
            AvailableNote::try_from(0 as usize).unwrap();
        let y: f64 = x.into();
        assert_eq!(y, LOWEST_NOTE);
        let x = AvailableNote::try_from(NUMBER_OF_AVAILABLE_NOTES);
        match x {
            Ok(value) => {
                panic!(
                    "Expected an error, got AvailableNote with value {}",
                    value.index
                )
            }
            Err(_error) => {}
        }
    }

    #[test]
    fn test_frequency_to_available_note() {
        let tolerance: f64 = 0.01;

        let freq: Frequency = Frequency::try_from(A_FREQUENCY + 2_f64).unwrap();
        let note: AvailableNote<NUMBER_OF_AVAILABLE_NOTES> = freq.try_into().unwrap();
        let freq_reconverted = TryInto::<f64>::try_into(note).unwrap();
        assert!(
            freq_reconverted - tolerance < A_FREQUENCY
                && freq_reconverted + tolerance > A_FREQUENCY
        );

        let freq: Frequency = Frequency::try_from(A_FREQUENCY*SEMI_TONE_FACTOR).unwrap();
        let note: AvailableNote<NUMBER_OF_AVAILABLE_NOTES> = freq.try_into().unwrap();
        let freq_reconverted = TryInto::<f64>::try_into(note).unwrap();
        assert!(
            freq_reconverted - tolerance > A_FREQUENCY
                || freq_reconverted + tolerance < A_FREQUENCY
        );
    }
}
