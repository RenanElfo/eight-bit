use crate::tone::Tone;

#[allow(dead_code)]
pub trait HasTone {
    fn get_tone(&self) -> Tone;

    fn set_tone(&mut self, tone: Tone);

    fn major_third(&self) -> Self
    where
        Self: Clone,
    {
        let mut major_third = self.clone();
        major_third.set_tone(major_third.get_tone().major_third().unwrap());
        return major_third;
    }
}

#[macro_export]
macro_rules! impl_has_tone {
    ($name: ty) => {
        type Tone = crate::tone::Tone;
        impl HasTone for $name {
            fn get_tone(&self) -> Tone {
                self.tone
            }

            fn set_tone(&mut self, tone: Tone) {
                self.tone = tone;
            }
        }
    };
}
