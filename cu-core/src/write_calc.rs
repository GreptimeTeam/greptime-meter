/// Trait representing how to calculate the byte count of a write request.
///
/// The `wcu!` macro uses this trait. See the documentation of the `cu-macros`
/// crate for details.
pub trait WriteCalc {
    fn byte_count(&self) -> u32;
}

impl WriteCalc for u32 {
    fn byte_count(&self) -> u32 {
        *self
    }
}

pub trait WriteCalculator<T>: Send + Sync {
    fn calc(&self, value: T) -> u32;
}
