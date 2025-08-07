mod audio;
mod waves;
use waves::ToAudio;
mod tone;
use builder_derive_macro::Setters;

#[derive(Debug, Setters)]
pub struct MyStruct {
    my_field_1: i32,
    my_field_2: u32,
}

fn main() {
    let x = MyStruct {
        my_field_1: 1,
        my_field_2: 2,
    }
    .with_my_field_1(3)
    .with_my_field_2(4);
    println!("{:?}", x);

    let tone = tone::Tone::try_from(880.0).unwrap();
    let test_pulse_builder: waves::PulseBuilder = waves::PulseBuilder::default()
        .with_tone(tone)
        .with_duration_ms(5000.0)
        .with_amplitude(2.0_f64.powf(12.0));
    println!("{:?}", test_pulse_builder);
    let test_pulse_1 = test_pulse_builder.clone().finalize().unwrap();
    let test_pulse_2 = test_pulse_builder
        .with_tone(tone.get_minor_third().unwrap())
        .finalize()
        .unwrap();
    let test_sine_builder: waves::SineBuilder = waves::SineBuilder::default()
        .with_tone(tone)
        .with_duration_ms(5000.0)
        .with_amplitude(2.0_f64.powf(12.0));
    let test_sine = test_sine_builder.finalize().unwrap();
    let audio_1 = test_pulse_1.to_audio().unwrap();
    let audio_2 = test_pulse_2.to_audio().unwrap();
    let audio_3 = test_sine.to_audio().unwrap();
    let audio = ((audio_1 / audio_2).unwrap() - audio_3).unwrap();

    audio.write_wav();
}
