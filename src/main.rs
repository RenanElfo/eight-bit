mod audio;
mod rythm;
use rythm::{Rythm, RythmBuilder};
mod utils;
use utils::build::Build;
mod time;
mod waves;

#[allow(unused_variables)]
fn main() {
    let freq = 110.0;
    let amplitude = 2.0_f64.powf(12.0);
    let sine = waves::SineBuilder::default()
        .with_tone(freq)
        .with_duration_ms(4000.0)
        .with_amplitude(amplitude);
    let mut rythm_1: Rythm<waves::Sine> = RythmBuilder::default()
        .with_tempo_bpm(54.0)
        .with_clamp(true)
        .with_decay(audio::basic_filters::Decay(400.0))
        .finalize()
        .unwrap();
    let mut rythm_2 = rythm_1.clone();
    let mut rythm_3 = rythm_1.clone();
    let mut rythm_4 = rythm_1.clone();
    rythm_1.hits_with_frequency(
        sine.clone().finalize().unwrap(),
        &[(0.75, "g3"), (0.75, "d3")],
    );
    rythm_2.hits_with_frequency(
        sine.clone().finalize().unwrap(),
        &[(-0.25, ""), (0.5, "b3"), (-0.25, ""), (0.5, "a3")],
    );
    rythm_3.hits_with_frequency(
        sine.clone().finalize().unwrap(),
        &[(-0.25, ""), (0.5, "d3"), (-0.25, ""), (0.5, "c#3")],
    );
    rythm_4.hits_with_frequency(
        sine.clone().finalize().unwrap(),
        &[(-0.25, ""), (0.5, "f#3"), (-0.25, ""), (0.5, "f#3")],
    );
    rythm_1.bis(1);
    rythm_2.bis(1);
    rythm_3.bis(1);
    rythm_4.bis(1);
    let audio = Into::<audio::Audio>::into(rythm_1) / Into::<audio::Audio>::into(rythm_2);
    // let audio = audio
    //     .filter_audio(audio::basic_filters::BitCruncher(8))
    //     .filter_audio(audio::basic_filters::SimpleDownsampler(4));
    // .filter_audio(audio::basic_filters::SimpleUpsampler(4));
    audio.write_wav();
}
