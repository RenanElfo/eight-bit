// use builder_derive_macro::{Finalize, Setters};

use crate::audio::ToAudio;
use crate::utils::build::Build;

// TODO: change Setters proc macro such that it works with generics
#[derive(Clone, Debug, PartialEq)]
pub struct RythmBuilder<T: ToAudio + Clone> {
    tempo_bpm: f64,
    // #[skip_setter]
    rythm: Vec<T>,
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
}

impl<T: ToAudio + Clone> Default for RythmBuilder<T> {
    fn default() -> Self {
        return Self {
            tempo_bpm: 60.0,
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
            rythm: vec![],
        });
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Rythm<T: ToAudio + Clone> {
    tempo_bpm: f64,
    rythm: Vec<T>,
}

#[allow(dead_code)]
impl<T: ToAudio + Clone> Rythm<T> {
    pub fn hit(mut self, _duration: f64, sound: T) {
        // let audio = sound.to_audio();
        self.rythm.push(sound);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::waves;

    #[test]
    fn common_use() {
        let x = RythmBuilder::default()
            .with_tempo_bpm(90.0)
            .finalize()
            .unwrap();
        x.hit(0.25, waves::SineBuilder::default().finalize().unwrap());
    }

    #[test]
    fn test_macro_attr() {
        let x: RythmBuilder<waves::Sine> = RythmBuilder {
            tempo_bpm: 60.0,
            rythm: vec![],
        }
        .with_tempo_bpm(45.0);
        assert_eq!(x.get_tempo_bpm(), &45.0);
        // .with_rythm(vec![1.0]);
    }
}
