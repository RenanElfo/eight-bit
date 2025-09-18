use builder_derive_macro::{Setters, Finalize};

#[derive(Clone, Debug, PartialEq, Setters, Finalize)]
pub struct RythmBuilder {
    tempo: f64,
    meter: (u16, u16),
}


