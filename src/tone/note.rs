use std::convert::{TryFrom, TryInto};

use super::standard_notes::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Note<const MAX_SIZE: usize> {
    index: usize,
}

impl TryFrom<usize> for Note<NUMBER_OF_AVAILABLE_NOTES> {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value >= NUMBER_OF_AVAILABLE_NOTES {
            return Err("Passed usize value is larger than maximum allowed.");
        }
        return Ok(Note { index: value });
    }
}

impl Into<f64> for Note<NUMBER_OF_AVAILABLE_NOTES> {
    fn into(self: Self) -> f64 {
        return AVAILABLE_NOTES[self.index];
    }
}

impl Into<Frequency> for Note<NUMBER_OF_AVAILABLE_NOTES> {
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

impl TryInto<Note<NUMBER_OF_AVAILABLE_NOTES>> for Frequency {
    type Error = &'static str;

    fn try_into(self: Self) -> Result<Note<NUMBER_OF_AVAILABLE_NOTES>, Self::Error> {
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
            Some(index) => return Note::try_from(index),
            None => return Err("Hello World"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_note_conversions() {
        let x: Note<NUMBER_OF_AVAILABLE_NOTES> =
            Note::try_from(0 as usize).unwrap();
        let y: f64 = x.into();
        assert_eq!(y, LOWEST_NOTE);
        let x = Note::try_from(NUMBER_OF_AVAILABLE_NOTES);
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
        let note: Note<NUMBER_OF_AVAILABLE_NOTES> = freq.try_into().unwrap();
        let freq_reconverted = TryInto::<f64>::try_into(note).unwrap();
        assert!(
            freq_reconverted - tolerance < A_FREQUENCY
                && freq_reconverted + tolerance > A_FREQUENCY
        );

        let freq: Frequency = Frequency::try_from(A_FREQUENCY * SEMI_TONE_FACTOR).unwrap();
        let note: Note<NUMBER_OF_AVAILABLE_NOTES> = freq.try_into().unwrap();
        let freq_reconverted = TryInto::<f64>::try_into(note).unwrap();
        assert!(
            freq_reconverted - tolerance > A_FREQUENCY
                || freq_reconverted + tolerance < A_FREQUENCY
        );

        let freq: Frequency = Frequency::try_from(100_000_f64).unwrap();
        let note = TryInto::<Note<NUMBER_OF_AVAILABLE_NOTES>>::try_into(freq);
        match note {
            Ok(value) => {
                panic!(
                    "Expected an error, got AvailableNote with value {}",
                    value.index
                )
            }
            Err(_error) => {}
        }
    }
}
