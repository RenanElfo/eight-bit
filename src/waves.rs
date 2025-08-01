use std::f64::consts::PI;

// use crate::audio;
use crate::tone;

// pub trait ToAudio {
//     fn to_audio(self, sampling_frequency: f64) -> audio::AudioBuilder;
// }

#[derive(Clone, Debug, PartialEq)]
pub struct PulseBuilder {
    tone: tone::Tone,
    amplitude: f64,
    duration_ms: f64,
    phase: f64,
    duty_cycle: f64,
}

impl Default for PulseBuilder {
    fn default() -> Self {
        return PulseBuilder {
            tone: tone::Tone::default(),
            amplitude: 1.0,
            duration_ms: 0.0,
            phase: 0.0,
            duty_cycle: 0.5,
        };
    }
}

#[allow(dead_code)]
impl PulseBuilder {
    pub fn new(
        tone: tone::Tone,
        amplitude: f64,
        duration_ms: f64,
        phase: f64,
        duty_cycle: f64,
    ) -> Self {
        return Self {
            tone,
            amplitude,
            duration_ms,
            phase,
            duty_cycle,
        };
    }

    pub fn with_tone(mut self, tone: tone::Tone) -> Self {
        self.tone = tone;
        return self;
    }

    pub fn with_amplitude(mut self, amplitude: f64) -> Self {
        self.amplitude = amplitude;
        return self;
    }

    pub fn with_duration_ms(mut self, duration: f64) -> Self {
        self.duration_ms = duration;
        return self;
    }

    pub fn with_rad_phase(mut self, phase: f64) -> Self {
        self.phase = phase;
        return self;
    }

    pub fn with_deg_phase(mut self, phase: f64) -> Self {
        self.phase = PI * phase / 180.0;
        return self;
    }

    pub fn with_duty_cycle(mut self, duty_cycle: f64) -> Self {
        self.duty_cycle = duty_cycle;
        return self;
    }
}

// impl ToAudio for Pulse {
//     fn to_audio(self, sampling_frequency: f64) -> audio::AudioBuilder {
//         return audio::AudioBuilder::new(vec![0], self.amplitude, sampling_frequency);
//     }
// }

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_file() {}

    // #[test]
    // fn test_default() {
    //     let x: Wave = Wave::default();
    //     if let tone::Tone::Pitch(frequency) = x.tone {
    //         assert_eq!(frequency, tone::A_FREQUENCY);
    //     }
    // }
}
