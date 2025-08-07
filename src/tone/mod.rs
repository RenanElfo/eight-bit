pub mod standard_notes;

pub use standard_notes::*;

#[derive(Clone, Debug, PartialEq)]
pub enum InvalidToneKind {
    NanFrequency,
    InfiniteFrequency,
    NegativeFrequency,
    OutOfBoundsNote,
    NoEquivalentNote,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InvalidTone {
    kind: InvalidToneKind,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AvailableTones<const MAX_SIZE: usize> {
    Pitch(f64),
    Note(usize),
}

pub type Tone = AvailableTones<NUMBER_OF_AVAILABLE_NOTES>;

impl Default for Tone {
    fn default() -> Self {
        return AvailableTones::Pitch(A_FREQUENCY)
            .as_note()
            .expect("A_FREQUENCY should have been a valid frequency");
    }
}

impl TryFrom<usize> for Tone {
    type Error = InvalidTone;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value >= NUMBER_OF_AVAILABLE_NOTES {
            return Err(InvalidTone {
                kind: InvalidToneKind::OutOfBoundsNote,
            });
        }
        return Ok(AvailableTones::Note(value));
    }
}

impl Into<f64> for Tone {
    fn into(self) -> f64 {
        match self {
            Self::Pitch(frequency) => {
                return frequency;
            }
            Self::Note(index) => {
                return AVAILABLE_NOTES[index];
            }
        }
    }
}

impl TryFrom<f64> for Tone {
    type Error = InvalidTone;

    fn try_from(frequency: f64) -> Result<Self, Self::Error> {
        if frequency < 0_f64 {
            return Err(InvalidTone {
                kind: InvalidToneKind::NegativeFrequency,
            });
        }
        if frequency.is_nan() {
            return Err(InvalidTone {
                kind: InvalidToneKind::NanFrequency,
            });
        }
        if frequency.is_infinite() {
            return Err(InvalidTone {
                kind: InvalidToneKind::InfiniteFrequency,
            });
        }
        return Ok(AvailableTones::Pitch(frequency));
    }
}

#[allow(dead_code)]
impl Tone {
    pub fn as_pitch(self) -> Self {
        match self {
            AvailableTones::Pitch(_frequency) => self,
            AvailableTones::Note(_index) => {
                let frequency: f64 = self.into();
                return AvailableTones::Pitch(frequency);
            }
        }
    }

    pub fn as_note(self) -> Result<Self, InvalidTone> {
        match self {
            AvailableTones::Pitch(frequency) => {
                let possible_notes_indices = AVAILABLE_NOTES
                    .into_iter()
                    .enumerate()
                    .filter(|(_index, note)| {
                        let distance = (note - frequency).abs();
                        return distance < LOWEST_NOTE;
                    })
                    .map(|(index, _note)| index)
                    .nth(0);
                match possible_notes_indices {
                    Some(index) => return AvailableTones::try_from(index),
                    None => {
                        return Err(InvalidTone {
                            kind: InvalidToneKind::NoEquivalentNote,
                        })
                    }
                }
            }
            AvailableTones::Note(_index) => Ok(self),
        }
    }
}

#[allow(dead_code)]
impl Tone {
    pub fn get_minor_third(&self) -> Result<Self, InvalidTone> {
        match self.clone() {
            Self::Pitch(frequency) => {
                let third = SEMI_TONE_FACTOR.powf(3.0) * frequency;
                return Ok(Tone::Pitch(third));
            }
            Self::Note(note) => {
                let third = note + 3;
                return Ok(Tone::try_from(third)?);
            }
        }
    }

    pub fn get_major_third(&self) -> Result<Self, InvalidTone> {
        match self.clone() {
            Self::Pitch(frequency) => {
                let third = SEMI_TONE_FACTOR.powf(4.0) * frequency;
                return Ok(Tone::Pitch(third));
            }
            Self::Note(note) => {
                let third = note + 4;
                return Ok(Tone::try_from(third)?);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
