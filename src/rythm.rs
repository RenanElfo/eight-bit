use builder_derive_macro::{Finalize, Setters};

use crate::audio::ToAudio;
use crate::utils::build::Build;

#[derive(Clone, Debug, PartialEq, Setters, Finalize)]
pub struct RythmBuilder {
    tempo_bpm: f64,
    #[skip_setter]
    rythm: Vec<f64>,
}

impl Default for RythmBuilder {
    fn default() -> Self {
        return Self {
            tempo_bpm: 60.0,
            rythm: vec![],
        };
    }
}

impl Build<Rythm, ()> for RythmBuilder {
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

    fn finalize(self) -> Result<Rythm, ()> {
        if let Result::Err(error) = self.validate() {
            return Err(error[0].clone());
        }
        return Ok(Rythm {
            tempo_bpm: self.tempo_bpm,
            rythm: vec![],
        });
    }
}

pub struct Rythm {
    tempo_bpm: f64,
    rythm: Vec<f64>,
}

#[allow(dead_code)]
impl Rythm {
    pub fn hit<T: ToAudio>(mut self, _duration: f64, sound: T) {
        let audio = sound.to_audio();
        self.rythm.push(0.0);
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
        let x = RythmBuilder {
            tempo_bpm: 60.0,
            rythm: vec![],
        }
        .with_tempo_bpm(45.0);
        assert_eq!(x.get_tempo_bpm(), &45.0);
        // .with_rythm(vec![1.0]);
    }
}
