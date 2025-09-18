mod pulse;
pub use pulse::*;
mod sine;
pub use sine::*;
mod triangle;
pub use triangle::*;
mod sawtooth;
pub use sawtooth::*;
mod noise;
pub use noise::*;

#[derive(Clone, Debug, PartialEq)]
pub enum InvalidWaveFormKind {
    NegativeDuration,
    NegativeDutyCycle,
    DutyCycleBiggerThanOne,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InvalidWaveForm {
    kind: InvalidWaveFormKind,
}

