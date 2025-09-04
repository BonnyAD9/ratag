pub use encoding::DecoderTrap;

use crate::{Error, Result};

mod skip;

pub use self::skip::*;

/// Trap decides what will happen on recoverable errors.
pub trait Trap {
    /// What to do if this error occurs. If this returns [`Err`], error is
    /// returned, otherwise the error is recovered and skiped.
    fn error(&self, err: Error) -> Result<()>;

    /// What to do with decoding errors.
    fn decoder_trap(&self) -> DecoderTrap;
}

pub(crate) trait TrapExt {
    fn res<T>(&self, res: Result<T>) -> Result<Option<T>>;
}

impl<T: Trap> TrapExt for T {
    fn res<U>(&self, res: Result<U>) -> Result<Option<U>> {
        match res {
            Err(e) => self.error(e).map(|_| None),
            Ok(r) => Ok(Some(r)),
        }
    }
}
