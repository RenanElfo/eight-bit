mod audio;
use audio::ToAudio;
mod rythm;
mod tone;
mod utils;
mod waves;

#[allow(unused_variables)]
fn main() {
    let tone = tone::Tone::try_from(110.0).unwrap();
    let test_pulse_builder: waves::PulseBuilder = waves::PulseBuilder::default()
        .with_tone(tone)
        .with_duration_ms(2500.0)
        .with_amplitude(2.0_f64.powf(12.0));
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
    let test_noise_builder_1: waves::NoiseBuilder = waves::NoiseBuilder::default()
        .with_duration_ms(2500.0)
        .with_amplitude(2.0_f64.powf(12.0));
    let test_noise_1 = test_noise_builder_1.finalize().unwrap();
    let test_noise_builder_2: waves::NoiseBuilder = waves::NoiseBuilder::default()
        .with_duration_ms(1250.0)
        .with_amplitude(0.5 * 2.0_f64.powf(12.0));
    let test_noise_2 = test_noise_builder_2.finalize().unwrap();
    let test_brown_noise_builder: waves::NoiseBuilder = waves::NoiseBuilder::default()
        .with_duration_ms(2500.0)
        .with_variant(waves::NoiseVariant::Brown)
        .with_amplitude(1.0 * 2.0_f64.powf(12.0));
    let test_blue_noise_builder: waves::NoiseBuilder = test_brown_noise_builder
        .clone()
        .with_variant(waves::NoiseVariant::Blue);
    let test_pink_noise_builder: waves::NoiseBuilder = test_brown_noise_builder
        .clone()
        .with_variant(waves::NoiseVariant::Pink);
    let test_violet_noise_builder: waves::NoiseBuilder = test_brown_noise_builder
        .clone()
        .with_variant(waves::NoiseVariant::Violet);
    let test_brown_noise = test_brown_noise_builder
        .with_amplitude(8.0 * 2.0_f64.powf(12.0))
        .finalize()
        .unwrap();
    let test_blue_noise = test_blue_noise_builder.finalize().unwrap();
    let test_pink_noise = test_pink_noise_builder.finalize().unwrap();
    let test_violet_noise = test_violet_noise_builder.finalize().unwrap();
    let audio_1 = test_pulse_1.to_audio().unwrap();
    let audio_2 = test_pulse_2.to_audio().unwrap();
    let audio_octavated = test_pulse_3.to_audio().unwrap();
    let audio_octavated_2 = test_pulse_4.to_audio().unwrap();
    let audio_3 = test_sine.to_audio().unwrap();
    let audio_4 = test_triangle.to_audio().unwrap();
    let audio_5 = test_sawtooth.to_audio().unwrap();
    let audio_6 = test_noise_1.to_audio().unwrap();
    let audio_7 = test_noise_2.to_audio().unwrap();
    let audio_8 = test_brown_noise.to_audio().unwrap();
    let audio_9 = test_blue_noise.to_audio().unwrap();
    let audio_10 = test_pink_noise.to_audio().unwrap();
    let audio_11 = test_violet_noise.to_audio().unwrap();
    // let audio = (audio_1 / audio_2).unwrap()
    //     - audio_octavated
    //     - audio_octavated_2
    //     - audio_3
    //     - audio_4
    //     - audio_5
    //     - audio_6
    //     - audio_7
    //     - audio_8;
    let audio = audio_8 - audio_10 - audio_6 - audio_9 - audio_11;
    // audio.write_wav();

    let mut freq = 110.0;
    let mut tone = tone::Tone::Pitch(freq);
    let mut amplitude = 2.0_f64.powf(12.0);
    let mut sine = waves::SineBuilder::default()
        .with_tone(tone)
        .with_duration_ms(5000.0)
        .with_amplitude(amplitude);
    let mut audio = sine.clone().finalize().unwrap().to_audio().unwrap();
    let sine_audio = audio.clone();
    let mut count = 1.0;
    while freq < 20_000.0 {
        count += 1.0;
        freq = 110.0 * count;
        tone = tone::Tone::Pitch(freq);
        amplitude /= 2.0;
        sine = sine.with_tone(tone).with_amplitude(amplitude);
        audio = (audio / sine.clone().finalize().unwrap().to_audio().unwrap()).unwrap();
    }
    audio.write_wav();

    let start_freq = 110.0;
    // let amplitude = 2.0_f64.powf(12.0);
    let amplitude = 1.0;
    let wave = waves::SineBuilder::default()
        .with_tone(tone::Tone::Pitch(start_freq))
        .with_duration_ms(5000.0)
        .with_amplitude(amplitude)
        .with_updater(Some(|sine: waves::SineBuilder, _index: usize| {
            let tone = sine.get_tone();
            let frequency: f64 = Into::<f64>::into(*tone);
            // println!("{}", frequency);
            let new_sine = sine.with_tone(tone::Tone::Pitch(frequency + 0.01));
            return new_sine.finalize().unwrap();
        }));
        // .with_updater(Some(|sine: waves::Sine, sample: usize| -> waves::Sine {
        //     return sine;
        // }));
    let wave_2 = wave.clone().with_rad_phase(-std::f64::consts::PI);
    // let wave_2 = wave.clone().with_amplitude(-amplitude);
    let audio_wave = wave.finalize().unwrap().to_audio().unwrap();
    let audio_wave_2 = wave_2.finalize().unwrap().to_audio().unwrap();
    let mut diff: Vec<f64> = vec![];
    let samples = audio_wave.clone().get_samples();
    let samples_2 = audio_wave_2.clone().get_samples();
    for i in 0..samples.len() {
        diff.push(samples[i] + samples_2[i]);
    }
    println!("{:?}", diff);
    // println!("{:?}", audio_wave.clone().get_samples());
    // println!("{:?}", audio_wave_2.clone().get_samples());
    // wave.finalize().unwrap().to_audio().unwrap().write_wav();
    // audio_wave.write_wav();
    // (audio_wave / audio_wave_2).unwrap().write_wav();
    // (sine_audio - audio).write_wav();
}
