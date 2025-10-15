pub const SEMI_TONE_FACTOR: f64 = f64::from_bits(4607450216769616227);

pub const C_0_FREQUENCY: f64 = f64::from_bits(4625295783300872534);
pub const D_0_FREQUENCY: f64 = f64::from_bits(4625859422914022178);
pub const E_0_FREQUENCY: f64 = f64::from_bits(4626492086988706426);
pub const F_0_FREQUENCY: f64 = f64::from_bits(4626836905701460669);
pub const G_0_FREQUENCY: f64 = f64::from_bits(4627589274320481790);
pub const A_0_FREQUENCY: f64 = f64::from_bits(4628433779541671936);
pub const B_0_FREQUENCY: f64 = f64::from_bits(4629381704602056984);

#[derive(PartialEq)]
pub enum PitchParserState {
    Note,
    ModifierOrOctave,
    Flat,
    Sharp,
    Octave(u8),
}

macro_rules! check_and_treat_digit {
    ($character: expr) => {{
        let digit = $character
            .to_digit(10)?
            .clamp(u8::MIN as u32, u8::MAX as u32);
        digit
    }};
}

// This could be changed to a regex, but I don't want to
// add another depency just to parse a simple string. Maybe
// this is something to consider later.
pub fn parse_note(note: &str) -> Option<f64> {
    type State = PitchParserState;
    let mut state = State::Note;
    let chars = note.chars();
    let mut frequency: Option<f64> = None;
    for character in chars {
        match state {
            State::Note => {
                let key = character.to_ascii_lowercase();
                frequency = Some(match key {
                    'c' => C_0_FREQUENCY,
                    'd' => D_0_FREQUENCY,
                    'e' => E_0_FREQUENCY,
                    'f' => F_0_FREQUENCY,
                    'g' => G_0_FREQUENCY,
                    'a' => A_0_FREQUENCY,
                    'b' => B_0_FREQUENCY,
                    _ => return None,
                });
                state = State::ModifierOrOctave;
            }
            State::ModifierOrOctave => {
                if character == 'b' {
                    frequency = frequency.map(|freq| freq / SEMI_TONE_FACTOR);
                    state = State::Flat;
                } else if character == '#' {
                    frequency = frequency.map(|freq| freq * SEMI_TONE_FACTOR);
                    state = State::Sharp;
                } else {
                    let digit = check_and_treat_digit!(character);
                    state = State::Octave(digit as u8);
                }
            }
            State::Flat => {
                if character == 'b' {
                    frequency = frequency.map(|freq| freq / SEMI_TONE_FACTOR);
                } else {
                    let digit = check_and_treat_digit!(character);
                    state = State::Octave(digit as u8);
                }
            }
            State::Sharp => {
                if character == '#' {
                    frequency = frequency.map(|freq| freq * SEMI_TONE_FACTOR);
                } else {
                    let digit = check_and_treat_digit!(character);
                    state = State::Octave(digit as u8);
                }
            }
            State::Octave(octave) => {
                let digit = check_and_treat_digit!(character);
                let new_octave = (octave as u32 * 10 + digit).clamp(u8::MIN as u32, u8::MAX as u32);
                state = State::Octave(new_octave as u8);
            }
        }
    }
    if let State::Octave(octave) = state {
        frequency = frequency.map(|freq| freq * 2.0_f64.powf(octave as f64));
    }
    return frequency;
}

pub fn parse_interval(relative_note: &str) -> Option<i32> {
    // "m7@3" => major seveth three octaves higher;
    // "tt@-2" => tritone two octaves lower;
    let mut split_note = relative_note.split('@');
    let interval = split_note.next()?;
    let interval = Interval::try_from(interval).ok()?;
    let octaves = split_note.next()?;
    let octaves = octaves
        .parse::<i32>()
        .ok()?
        .clamp(i8::MIN as i32, i8::MAX as i32);
    return Some(12 * octaves as i32 + interval.to_semitones() as i32);
}

#[derive(Clone, Debug, PartialEq)]
pub enum Interval {
    Unison,
    MinorSecond,
    MajorSecond,
    MinorThird,
    MajorThird,
    PerfectForth,
    Tritone,
    PerfectFifth,
    MinorSixth,
    MajorSixth,
    MinorSeventh,
    MajorSeventh,
}

impl TryFrom<&str> for Interval {
    type Error = ();

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        return Ok(match string {
            "root" => Self::Unison,
            "m2" => Self::MinorSecond,
            "M2" => Self::MajorSecond,
            "m3" => Self::MinorThird,
            "M3" => Self::MajorThird,
            "p4" => Self::PerfectForth,
            "3t" => Self::Tritone,
            "p5" => Self::PerfectFifth,
            "m6" => Self::MinorSixth,
            "M6" => Self::MajorSixth,
            "m7" => Self::MinorSeventh,
            "M7" => Self::MajorSeventh,
            _ => return Err(()),
        });
    }
}

impl Interval {
    pub const fn to_semitones(&self) -> i8 {
        match self {
            Self::Unison => 0,
            Self::MinorSecond => 1,
            Self::MajorSecond => 2,
            Self::MinorThird => 3,
            Self::MajorThird => 4,
            Self::PerfectForth => 5,
            Self::Tritone => 6,
            Self::PerfectFifth => 7,
            Self::MinorSixth => 8,
            Self::MajorSixth => 9,
            Self::MinorSeventh => 10,
            Self::MajorSeventh => 11,
        }
    }
}

macro_rules! relative_tone {
    ($self: expr, $semi_notes: expr) => {{
        let new_tone = SEMI_TONE_FACTOR.powf($semi_notes as f64) * $self.get_tone();
        new_tone
    }};
}

#[allow(dead_code)]
pub trait HasTone {
    fn get_tone(&self) -> f64;

    fn set_tone(&mut self, tone: f64);

    fn octavate(&self, octaves: i32) -> f64 {
        relative_tone!(self, 12 * octaves)
    }

    fn parse_note(&self, note: &str) -> Option<f64> {
        Some(
            parse_interval(note)
                .map(|semi_tones| relative_tone!(self, semi_tones))
                .unwrap_or(parse_note(note)?),
        )
    }

    fn parse_and_set_tone(&mut self, note: &str) -> Result<(), ()> {
        let new_tone = self.parse_note(note).ok_or(())?;
        Ok(self.set_tone(new_tone))
    }
}

#[macro_export]
macro_rules! impl_has_tone {
    ($name: ty) => {
        impl HasTone for $name {
            fn get_tone(&self) -> f64 {
                self.tone
            }

            fn set_tone(&mut self, tone: f64) {
                self.tone = tone;
            }
        }
    };
}
