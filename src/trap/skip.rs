use encoding::DecoderTrap;

use crate::{Error, Result, trap::Trap};

pub struct Skip;

impl Trap for Skip {
    fn error(&self, _: Error) -> Result<()> {
        Ok(())
    }

    fn decoder_trap(&self) -> DecoderTrap {
        DecoderTrap::Replace
    }
}
