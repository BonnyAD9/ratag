pub use encoding::DecoderTrap;

use crate::{Error, Result};

mod skip;

pub use self::skip::*;

pub trait Trap {
    fn error(&self, err: Error) -> Result<()>;

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
