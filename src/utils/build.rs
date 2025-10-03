pub trait Build {
    type Output;
    type Error;

    fn validate(&self) -> Result<(), Vec<Self::Error>> {
        return Ok(());
    }

    fn finalize(self) -> Result<Self::Output, Self::Error>;
}
