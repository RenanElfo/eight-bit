use std::collections::VecDeque;

use crate::audio::{basic_filters::Decay, Audio};
use crate::time::has_duration::HasDuration;
use crate::utils::build::Build;
use crate::waves::traits::has_tone::HasTone;

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
    EightNoteTriplet,
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
            Self::EightNoteTriplet => 1.0 / 12.0,
            Self::EigthNote => 1.0 / 8.0,
            Self::SixteenthNote => 1.0 / 16.0,
            Self::ThirtySecondNote => 1.0 / 32.0,
            Self::Custom(factor) => *factor,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RythmBuilder<T: Into<Audio> + Clone> {
    tempo_bpm: f64,
    beat_type: Beat,
    rythm: VecDeque<RythmElement<T>>,
    clamp: bool,
    decay: Option<Decay>,
}

#[allow(dead_code)]
impl<T: Into<Audio> + Clone> RythmBuilder<T> {
    pub fn get_tempo_bpm(&self) -> f64 {
        return self.tempo_bpm;
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

    pub fn get_clamp(&self) -> bool {
        return self.clamp;
    }

    pub fn with_clamp(mut self, clamp: bool) -> Self {
        self.clamp = clamp;
        return self;
    }

    pub fn with_decay(mut self, decay: Decay) -> Self {
        self.decay = Some(decay);
        return self;
    }
}

impl<T: Into<Audio> + Clone> Default for RythmBuilder<T> {
    fn default() -> Self {
        return Self {
            tempo_bpm: 60.0,
            beat_type: Beat::default(),
            rythm: VecDeque::new(),
            clamp: false,
            decay: None,
        };
    }
}

impl<T: Into<Audio> + Clone> Build for RythmBuilder<T> {
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
            clamp: false,
            decay: self.decay,
        });
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Rythm<T: Into<Audio>> {
    tempo_bpm: f64,
    beat_type: Beat,
    hit_start_time_ms: f64,
    rythm: VecDeque<RythmElement<T>>,
    clamp: bool,
    decay: Option<Decay>,
}

macro_rules! hit {
    ($rythm: expr, $sound: expr, $relative_duration: expr, $($note: expr)?, $($duration: expr)?) => {{
        let cloned = $sound.clone();
        $rythm.hit($relative_duration, cloned);
        //
    }};
}

#[allow(dead_code)]
fn test<T>(mut rythm: Rythm<T>, sound: T)
where
    T: Into<Audio> + Clone,
{
    hit!(rythm, sound, 0.5,,);
}

#[allow(dead_code)]
impl<T: Into<Audio>> Rythm<T> {
    pub fn hit(&mut self, duration: f64, sound: T) {
        if duration < 0.0 {
            self.rythm.push_back(RythmElement::Rest(Rest {
                relative_duration: -duration,
            }));
        } else if duration > 0.0 {
            self.rythm.push_back(RythmElement::Hit(Hit {
                relative_duration: duration,
                wave: sound,
            }));
        }
    }

    pub fn hits_with_frequency(&mut self, root_sound: T, notes: &[(f64, &str)])
    where
        T: HasTone + Clone,
    {
        for (duration, note) in notes {
            let mut cloned_sound = root_sound.clone();
            let parse_result = cloned_sound.parse_and_set_tone(note);
            match parse_result {
                Ok(()) => self.hit(*duration, cloned_sound),
                Err(()) => self.hit(-(*duration).abs(), cloned_sound),
            }
        }
    }

    pub fn hits_with_duration(&mut self, root_sound: T, durations: &[(f64, f64)])
    where
        T: HasDuration + Clone,
    {
        for (hit_duration, sound_duration) in durations {
            let mut cloned_sound = root_sound.clone();
            cloned_sound.set_duration_ms(*sound_duration);
            self.hit(*hit_duration, cloned_sound);
        }
    }

    pub fn hits_with_matching_duration(&mut self, root_sound: T, durations: &[f64])
    where
        T: HasDuration + Clone,
    {
        let durations: Vec<_> = durations
            .iter()
            .map(|duration| (*duration, *duration))
            .collect();
        self.hits_with_duration(root_sound, &durations);
    }
}

// impl<T: Into<Audio>> Rythm<T> {}

#[allow(dead_code)]
impl<T: Into<Audio> + Clone> Rythm<T> {
    pub fn bis(&mut self, repetitions: usize) {
        let original: Vec<_> = self.rythm.iter().cloned().collect();
        self.rythm.reserve(self.rythm.len() * repetitions);
        for _ in 0..repetitions {
            self.rythm.extend(original.iter().cloned());
        }
    }

    pub fn len(&self) -> f64 {
        self.rythm
            .iter()
            .map(|element| element.relative_duration())
            .reduce(std::ops::Add::add)
            .unwrap_or(0.0)
    }
}

#[allow(dead_code)]
impl<T: Into<Audio>> Rythm<T> {
    fn hit_duration_ms(&self, hit: &RythmElement<T>) -> f64 {
        let ms_per_min = 60.0 * 1000.0;
        let whole_notes_per_minute = self.tempo_bpm * self.beat_type.duration_factor();
        let whole_note_duration_milliseconds = ms_per_min * (1.0 / whole_notes_per_minute);
        whole_note_duration_milliseconds * hit.relative_duration()
    }
}

impl<T: Into<Audio>> Iterator for Rythm<T> {
    type Item = Audio;

    fn next(&mut self) -> Option<Self::Item> {
        let hit = self.rythm.pop_front()?;
        let duration = Self::hit_duration_ms(self, &hit);
        let hit_start_time_ms = self.hit_start_time_ms;
        self.hit_start_time_ms += duration;
        let wave = hit.wave();
        match wave {
            Some(wave) => {
                let mut audio = wave.into();
                if let Some(decay) = &self.decay {
                    audio = audio.filter_audio(decay.clone());
                }
                audio.set_duration_ms(duration);
                audio.milliseconds_left_pad(hit_start_time_ms);
                Some(audio)
            }
            None => self.next(),
        }
    }
}

impl<T: Into<Audio>> Into<Audio> for Rythm<T> {
    fn into(self) -> Audio {
        self.reduce(|acc, audio| acc / audio)
            .unwrap_or(Audio::default())
    }
}
