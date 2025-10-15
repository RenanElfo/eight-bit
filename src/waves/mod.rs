mod pulse;
pub use pulse::*;
mod sine;
pub use sine::*;
mod triangle;
#[allow(unused_imports)]
pub use triangle::*;
mod sawtooth;
#[allow(unused_imports)]
pub use sawtooth::*;
mod noise;
#[allow(unused_imports)]
pub use noise::*;
pub mod traits;

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
