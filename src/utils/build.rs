pub trait Build<T, E> {
    fn validate(&self) -> Result<(), Vec<E>> {
        return Ok(());
    }

    fn finalize(self) -> Result<T, E>;
}
