// use builder_derive_macro::{Finalize, Setters};

use crate::audio::{Audio, ToAudio};
use crate::utils::build::Build;

mod hit;
use hit::{Hit, Rest, RythmElement};
mod tests;

#[allow(dead_code)]
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Beat {
    WholeNote,
    HalfNote,
    #[default]
    QuarterNote,
    QuarterNoteTriplet,
    EigthNote,
    SixteenthNote,
    ThirtySecondNote,
}

impl Beat {
    fn duration_factor(&self) -> f64 {
        match self {
            Self::WholeNote => 1.0,
            Self::HalfNote => 0.5,
            Self::QuarterNote => 0.25,
            Self::QuarterNoteTriplet => 1.0 / 12.0,
            Self::EigthNote => 1.0 / 8.0,
            Self::SixteenthNote => 1.0 / 16.0,
            Self::ThirtySecondNote => 1.0 / 32.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RythmBuilder<T: ToAudio + Clone> {
    tempo_bpm: f64,
    beat_type: Beat,
    // #[skip_setter]
    rythm: Vec<RythmElement<T>>,
}

#[allow(dead_code)]
impl<T: ToAudio + Clone> RythmBuilder<T> {
    pub fn get_tempo_bpm(&self) -> &f64 {
        return &self.tempo_bpm;
    }

    pub fn with_tempo_bpm(mut self, tempo_bpm: f64) -> Self {
        self.tempo_bpm = tempo_bpm;
        return self;
    }

    pub fn get_beat_type(&self) -> &Beat {
        return &self.beat_type;
    }

    pub fn with_beat_type(mut self, beat_type: Beat) -> Self {
        self.beat_type = beat_type;
        return self;
    }
}

impl<T: ToAudio + Clone> Default for RythmBuilder<T> {
    fn default() -> Self {
        return Self {
            tempo_bpm: 60.0,
            beat_type: Beat::default(),
            rythm: vec![],
        };
    }
}

impl<T: ToAudio + Clone> Build<Rythm<T>, ()> for RythmBuilder<T> {
    fn validate(&self) -> Result<(), Vec<()>> {
        let mut possible_errors: Vec<_> = vec![];
        let tempo = self.tempo_bpm;
        if tempo < 0.0 || tempo.is_nan() || tempo.is_infinite() {
            possible_errors.push(());
        }
        if !possible_errors.is_empty() {
            return Err(possible_errors);
        };
        return Ok(());
    }

    fn finalize(self) -> Result<Rythm<T>, ()> {
        if let Result::Err(error) = self.validate() {
            return Err(error[0].clone());
        }
        return Ok(Rythm {
            tempo_bpm: self.tempo_bpm,
            beat_type: self.beat_type,
            rythm: vec![],
        });
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Rythm<T: ToAudio> {
    tempo_bpm: f64,
    beat_type: Beat,
    rythm: Vec<RythmElement<T>>,
}

#[allow(dead_code)]
impl<T: ToAudio> Rythm<T> {
    pub fn hit(mut self, duration: f64, _sound: T) {
        self.rythm.push(RythmElement::Rest(Rest {
            relative_duration: duration,
        }));
    }
}

#[allow(dead_code)]
impl<T: ToAudio> Rythm<T> {
    fn hit_duration(&self, hit: RythmElement<T>) -> f64 {
        let ms_per_min = 60.0 * 1000.0;
        let whole_notes_per_minute = self.tempo_bpm * self.beat_type.duration_factor();
        let whole_note_duration_milliseconds = ms_per_min * (1.0 / whole_notes_per_minute);
        whole_note_duration_milliseconds * hit.relative_duration()
    }

    fn bla(mut self, duration: f64, _sound: T) {
        self.rythm.push(RythmElement::Rest(Rest {
            relative_duration: duration,
        }));
    }
}

impl<T: ToAudio> Iterator for Rythm<T> {
    type Item = Audio;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

// impl<A: ToAudio> Extend<RythmElement<A>> for Rythm<A> {
//     fn extend<T: IntoIterator<Item = RythmElement<A>>>(&mut self, iter: T) {
//         for elem in iter {
//             self.hit(RythmElement::Rest(Rest { relative_duration: 0.0 }));
//         }
//     }
// }
