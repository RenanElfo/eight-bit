use super::Audio;

// #[allow(dead_code)]
// pub trait ToAudio {
//     fn to_audio(self) -> Result<Audio, InvalidAudio>;
// }

#[allow(dead_code)]
pub trait FilterAudio {
    fn filter(self, audio: Audio) -> Audio;
}
