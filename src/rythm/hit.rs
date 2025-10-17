use crate::audio::Audio;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum RythmElement<T: Into<Audio>> {
    Rest(Rest),
    Hit(Hit<T>),
}

impl<T: Into<Audio>> RythmElement<T> {
    pub fn relative_duration(&self) -> f64 {
        match self {
            Self::Rest(rest) => rest.relative_duration,
            Self::Hit(hit) => hit.relative_duration,
        }
    }

    pub fn wave(self) -> Option<T> {
        match self {
            Self::Rest(_rest) => None,
            Self::Hit(hit) => Some(hit.wave),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Rest {
    pub relative_duration: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Hit<T: Into<Audio>> {
    pub relative_duration: f64,
    pub wave: T,
}
