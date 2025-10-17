mod audio;
mod rythm;
use rythm::{Rythm, RythmBuilder};
mod utils;
use utils::build::Build;
mod time;
mod waves;

fn main() {
    let triangle = waves::TriangleBuilder::default().with_duration_ms(4000.0);
    let pulse = waves::PulseBuilder::default().with_duration_ms(4000.0);
    let mut bass: Rythm<waves::Triangle> = RythmBuilder::default()
        .with_tempo_bpm(54.0)
        .with_clamp(true)
        .with_decay(audio::basic_filters::Decay(400.0))
        .finalize()
        .unwrap();
    let mut harmony_1: Rythm<waves::Pulse> = RythmBuilder::default()
        .with_tempo_bpm(54.0)
        .with_clamp(true)
        .with_decay(audio::basic_filters::Decay(400.0))
        .finalize()
        .unwrap();
    let mut harmony_2 = harmony_1.clone();
    let mut harmony_3 = harmony_1.clone();
    let mut melody_1 = harmony_1.clone();
    let mut melody_2 = harmony_1.clone();
    bass.hits_with_frequency(
        triangle.clone().with_amplitude(4.0).finalize().unwrap(),
        &[(0.75, "g3"), (0.75, "d3")],
    );
    bass.bis(7);
    bass.hits_with_frequency(
        triangle.clone().with_amplitude(4.0).finalize().unwrap(),
        &[
            (0.75, "f#3"),
            // (0.74, "f#3"),
            (0.75, "b2"),
            (0.75, "e3"),
            (0.75, "e3"),
            (0.75, "d3"),
            (0.75, "a2"),
            (0.75, "d3"),
            (0.75, "d3"),
            (0.75, "d3"),
            (0.75, "d3"),
            (0.75, "d3"),
            (0.75, "d3"),
            (0.75, "d3"),
            (0.75, "d3"),
            (0.75, "d3"),
            (0.75, "e3"),
            (0.75, "f#3"),
            (0.75, "b2"),
            (0.75, "e3"),
            (0.75, "e3"),
            (0.75, "e3"),
            (0.75, "a3"),
            (0.75, "d3"),
            (0.75, "g3"),
            (0.75, "d3"),
            (0.75, "g3"),
            (0.75, "d3"),
            (0.75, "g3"),
            (0.75, "d3"),
            (0.75, "g3"),
            (0.75, "d3"),
            (0.75, "g3"),
            (0.75, "d3"),
            (0.75, "g3"),
            (0.75, "d3"),
            (0.75, "g3"),
            (0.75, "d3"),
            (0.75, "g3"),
            (0.75, "d3"),
            (0.75, "f#3"),
            (0.75, "b2"),
            (0.75, "e3"),
            (0.75, "e3"),
            (0.75, "d3"),
            (0.75, "a2"),
            (0.75, "d3"),
            (0.75, "d3"),
            (0.75, "d3"),
            (0.75, "d3"),
            (0.75, "d3"),
            (0.75, "d3"),
            (0.75, "d3"),
            (0.75, "d3"),
            (0.75, "d3"),
            (0.75, "e3"),
            (0.75, "e3"),
            (0.75, "e3"),
            (0.75, "e3"),
            (0.75, "e3"),
            (0.75, "e3"),
            (0.75, "a3"),
            (0.75, "d3"),
        ],
    );
    harmony_1.hits_with_frequency(
        pulse.clone().finalize().unwrap(),
        &[
            (-0.25, ""),
            // (-0.24, ""),
            (0.5, "b4"),
            (-0.25, ""),
            (0.5, "a4"),
            // (-0.01, ""),
        ],
    );
    harmony_1.bis(8);
    harmony_1.hits_with_frequency(
        pulse.clone().finalize().unwrap(),
        &[
            (-0.25, ""),
            // (-0.24, ""),
            (0.5, "b4"),
            (-0.25, ""),
            (0.5, "a4"),
            // (-0.01, ""),
        ],
    );
    harmony_2.hits_with_frequency(
        pulse.clone().finalize().unwrap(),
        &[
            (-0.25, ""),
            // (-0.245, ""),
            (0.5, "d5"),
            (-0.25, ""),
            (0.5, "c#5"),
            // (-0.005, ""),
        ],
    );
    harmony_3.hits_with_frequency(
        pulse.clone().finalize().unwrap(),
        &[(-0.25, ""), (0.5, "f#5"), (-0.25, ""), (0.5, "f#5")],
    );
    harmony_2.bis(8);
    harmony_3.bis(8);
    melody_1.hits_with_frequency(
        pulse.clone().finalize().unwrap(),
        &[
            (-13.0 / 4.0, ""),
            (0.25, "f#6"),
            (0.25, "a6"),
            (0.25, "g6"),
            (0.25, "f#6"),
            (0.25, "c#6"),
            (0.25, "b5"),
            (0.25, "c#6"),
            (0.25, "d6"),
            (0.75, "a5"),
            (0.75, "f#5"),
        ],
    );
    melody_2.hits_with_frequency(
        pulse.clone().finalize().unwrap(),
        &[
            (-(melody_1.len() + (10.0 / 4.0)), ""),
            (0.25, "f#6"),
            (0.25, "a6"),
            (0.25, "g6"),
            (0.25, "f#6"),
            (0.25, "c#6"),
            (0.25, "b5"),
            (0.25, "c#6"),
            (0.25, "d6"),
            (0.75, "a5"),
            (0.75, "c#6"),
            (0.75, "f#6"),
            (0.75, "e5"),
            (0.75, "e5"),
            (0.75, "e5"),
            (0.25, "a5"),
            (0.25, "b5"),
            (0.25, "c6"),
            (0.25, "e6"),
            (0.25, "d6"),
            (0.25, "b5"),
            (0.25, "d6"),
            (0.25, "c6"),
            (0.25, "b5"),
            (0.75, "d6"),
            (0.5, "d6"),
            (0.25, "d6"),
            (0.25, "e6"),
            (0.25, "f6"),
            (0.25, "g6"),
            (0.25, "a6"),
            (0.25, "c6"),
            (0.25, "d6"),
            (0.25, "e6"),
            (0.25, "d6"),
            (0.25, "b5"),
            (0.75, "d6"),
            (0.5, "d6"),
            (0.25, "d6"),
            (0.75, "g6"),
            (0.75, "f#6"),
            (0.25, "b5"),
            (0.25, "a5"),
            (0.25, "b5"),
            (0.25, "c#6"),
            (0.25, "d6"),
            (0.25, "e6"),
            (0.25, "c#6"),
            (0.25, "d6"),
            (0.25, "e6"),
            (0.75, "f#5"),
            (0.75, "c6"),
            (0.75, "d6"),
            (-3.25, ""),
            (0.25, "f#6"),
            (0.25, "a6"),
            (0.25, "g6"),
            (0.25, "f#6"),
            (0.25, "c#6"),
            (0.25, "b5"),
            (0.25, "c#6"),
            (0.25, "d6"),
            (0.75, "a5"),
            (0.75, "f#5"),
            (0.75, "f#5"),
            (0.75, "f#5"),
            (0.75, "f#5"),
            (-0.25, ""),
            (0.25, "f#6"),
            (0.25, "a6"),
            (0.25, "g6"),
            (0.25, "f#6"),
            (0.25, "c#6"),
            (0.25, "b5"),
            (0.25, "c#6"),
            (0.25, "d6"),
            (0.75, "a5"),
            (0.75, "c#6"),
            (0.75, "f#6"),
            (0.75, "e5"),
            (0.75, "e5"),
            (0.75, "e5"),
            (0.25, "a5"),
            (0.25, "b5"),
            (0.25, "c6"),
            (0.25, "e6"),
            (0.25, "d6"),
            (0.25, "b5"),
            (0.25, "d6"),
            (0.25, "c6"),
            (0.25, "b5"),
            (0.75, "d6"),
            (0.5, "d6"),
            (0.25, "d6"),
            (0.25, "e6"),
            (0.25, "f6"),
            (0.25, "g6"),
            (0.25, "a6"),
            (0.25, "c6"),
            (0.25, "d6"),
            (0.25, "e6"),
            (0.25, "d6"),
            (0.25, "b5"),
            (0.75, "d6"),
            (0.5, "d6"),
            (0.25, "d6"),
            (0.75, "g6"),
            (0.75, "f6"),
            (0.25, "b5"),
            (0.25, "c6"),
            (0.25, "f6"),
            (0.25, "e6"),
            (0.25, "d6"),
            (0.25, "c6"),
            (0.25, "e6"),
            (0.25, "d6"),
            (0.25, "c6"),
            (0.75, "f5"),
            (0.75, "c6"),
            (0.75, "d6"),
        ],
    );
    println!("{}", bass.len() * 4.0 / 3.0);
    let audio_1: audio::Audio = bass.into();
    let audio_2: audio::Audio = harmony_1.into();
    let audio_3: audio::Audio = harmony_2.into();
    let audio_4: audio::Audio = harmony_3.into();
    let audio_5: audio::Audio = melody_1.into();
    println!("{}", melody_2.len() * 4.0 / 3.0);
    let audio_6: audio::Audio = melody_2.into();
    let audio = audio_1 / audio_2 / audio_3 / audio_4 / audio_5 / audio_6;
    // let audio = audio.filter_audio(audio::basic_filters::BitCruncher(12));
    audio.write_wav();
}
