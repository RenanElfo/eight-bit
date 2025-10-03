use std::collections::VecDeque;

// use builder_derive_macro::{Finalize, Setters};

use crate::audio::{Audio, InvalidAudio, traits::ToAudio};
use crate::utils::build::Build;

mod hit;
use hit::{Rest, RythmElement};
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
    Custom(f64),
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
            Self::Custom(factor) => *factor,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RythmBuilder<T: ToAudio + Clone> {
    tempo_bpm: f64,
    beat_type: Beat,
    // #[skip_setter]
    rythm: VecDeque<RythmElement<T>>,
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
            rythm: VecDeque::new(),
        };
    }
}

impl<T: ToAudio + Clone> Build for RythmBuilder<T> {
    type Output = Rythm<T>;
    type Error = ();

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
            hit_start_time_ms: 0.0,
            rythm: VecDeque::new(),
            largest_sampling_frequency: 0.0,
        });
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Rythm<T: ToAudio> {
    tempo_bpm: f64,
    beat_type: Beat,
    hit_start_time_ms: f64,
    rythm: VecDeque<RythmElement<T>>,
    largest_sampling_frequency: f64,
}

#[allow(dead_code)]
impl<T: ToAudio> Rythm<T> {
    pub fn hit(&mut self, duration: f64, _sound: T) {
        self.rythm.push_back(RythmElement::Rest(Rest {
            relative_duration: duration,
        }));
    }
}

#[allow(dead_code)]
impl<T: ToAudio + Clone> Rythm<T> {
    pub fn bis(&mut self) {
        let mut repeated = self.rythm.clone();
        self.rythm.append(&mut repeated);
    }
}

#[allow(dead_code)]
impl<T: ToAudio> Rythm<T> {
    fn hit_duration_ms(&self, hit: &RythmElement<T>) -> f64 {
        let ms_per_min = 60.0 * 1000.0;
        let whole_notes_per_minute = self.tempo_bpm * self.beat_type.duration_factor();
        let whole_note_duration_milliseconds = ms_per_min * (1.0 / whole_notes_per_minute);
        whole_note_duration_milliseconds * hit.relative_duration()
    }
}

impl<T: ToAudio> Iterator for Rythm<T> {
    type Item = Audio;

    fn next(&mut self) -> Option<Self::Item> {
        let hit = self.rythm.pop_front()?;
        let duration = Self::hit_duration_ms(self, &hit);
        let hit_start_time_ms = self.hit_start_time_ms;
        self.hit_start_time_ms += duration;
        let wave = hit.wave();
        match wave {
            Some(wave) => {
                let mut audio = wave.to_audio().unwrap();
                audio.milliseconds_left_pad(hit_start_time_ms);
                Some(audio)
            }
            None => self.next(),
        }
    }
}

impl<T: ToAudio> ToAudio for Rythm<T> {
    fn to_audio(self) -> Result<Audio, InvalidAudio> {
        let audio_results = self.map(|hit| hit.to_audio());
        let mut audios: Vec<Audio> = vec![];
        for result in audio_results {
            audios.push(result?);
        }
        Ok(audios
            .into_iter()
            .reduce(|acc, audio| acc - audio)
            .unwrap_or(Audio::default()))
    }
}
