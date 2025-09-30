use crate::audio::ToAudio;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum RythmElement<T: ToAudio> {
    Rest(Rest),
    Hit(Hit<T>),
}

impl<T: ToAudio> RythmElement<T> {
    pub fn relative_duration(&self) -> f64 {
        match self {
            RythmElement::Rest(rest) => rest.relative_duration,
            RythmElement::Hit(hit) => hit.relative_duration,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Rest {
    pub relative_duration: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Hit<T: ToAudio> {
    pub relative_duration: f64,
    pub wave: T,
}
