use encoding::DecoderTrap;

use crate::{Error, Result, trap::Trap};

/// Trap that skips all errors.
pub struct Warn;

impl Trap for Warn {
    fn error(&self, e: Error) -> Result<()> {
        eprintln!("warning: {e}");
        Ok(())
    }

    fn decoder_trap(&self) -> DecoderTrap {
        DecoderTrap::Replace
    }
}
