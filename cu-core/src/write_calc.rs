/// Calculate the byte count of a write request.
pub trait WriteCalc {
    fn byte_count(&self) -> u32;
}

impl WriteCalc for u32 {
    fn byte_count(&self) -> u32 {
        *self
    }
}
