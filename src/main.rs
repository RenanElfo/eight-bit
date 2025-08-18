mod audio;
use audio::ToAudio;
mod rythm;
mod tone;
mod waves;

fn main() {
    let tone = tone::Tone::try_from(110.0).unwrap();
    let test_pulse_builder: waves::PulseBuilder = waves::PulseBuilder::default()
        .with_tone(tone)
        .with_duration_ms(2500.0)
        .with_amplitude(2.0_f64.powf(12.0));
    println!("{:?}", test_pulse_builder);
    let test_pulse_1 = test_pulse_builder.clone().finalize().unwrap();
    let test_pulse_2 = test_pulse_builder
        .clone()
        .with_tone(tone.minor_third().unwrap())
        .finalize()
        .unwrap();
    let test_pulse_3 = test_pulse_builder
        .clone()
        .with_duration_ms(1250.0)
        .with_tone(tone.octavate(-1).unwrap())
        .finalize()
        .unwrap();
    let test_pulse_4 = test_pulse_builder
        .with_duration_ms(1250.0)
        .with_tone(tone.octavate(1).unwrap())
        .finalize()
        .unwrap();
    let test_sine_builder: waves::SineBuilder = waves::SineBuilder::default()
        .with_tone(tone)
        .with_duration_ms(2500.0)
        .with_amplitude(2.0_f64.powf(12.0));
    let test_sine = test_sine_builder.finalize().unwrap();
    let test_triangle_builder: waves::TriangleBuilder = waves::TriangleBuilder::default()
        .with_tone(tone)
        .with_duration_ms(2500.0)
        .with_amplitude(2.0_f64.powf(12.0));
    let test_triangle = test_triangle_builder.finalize().unwrap();
    let test_sawtooth_builder: waves::SawtoothBuilder = waves::SawtoothBuilder::default()
        .with_tone(tone)
        .with_duration_ms(2500.0)
        .with_amplitude(2.0_f64.powf(12.0));
    let test_sawtooth = test_sawtooth_builder.finalize().unwrap();
    let audio_1 = test_pulse_1.to_audio().unwrap();
    let audio_2 = test_pulse_2.to_audio().unwrap();
    let audio_octavated = test_pulse_3.to_audio().unwrap();
    let audio_octavated_2 = test_pulse_4.to_audio().unwrap();
    let audio_3 = test_sine.to_audio().unwrap();
    let audio_4 = test_triangle.to_audio().unwrap();
    let audio_5 = test_sawtooth.to_audio().unwrap();
    let audio = ((((((audio_1 / audio_2).unwrap() - audio_octavated).unwrap()
        - audio_octavated_2)
        .unwrap()
        - audio_3)
        .unwrap()
        - audio_4)
        .unwrap()
        - audio_5)
        .unwrap();

    audio.write_wav();
}
