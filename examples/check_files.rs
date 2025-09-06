use std::{cell::Cell, path::Path, rc::Rc, time::Duration};

use encoding::DecoderTrap;
use ratag::{Error, Result, TagStore, read_tag_from_file, trap::Trap};

#[derive(Default)]
struct TrackTrap(Rc<Cell<bool>>);

impl Trap for TrackTrap {
    fn error(&self, err: Error) -> Result<()> {
        self.0.set(true);
        eprintln!("warning: {err}");
        Ok(())
    }

    fn decoder_trap(&self) -> DecoderTrap {
        DecoderTrap::Replace
    }
}

#[derive(Default)]
struct TrackStore {
    length: Option<Duration>,
}

impl TagStore for TrackStore {
    fn stores_data(&self, _typ: ratag::DataType) -> bool {
        true
    }

    fn set_length(&mut self, length: Option<Duration>) {
        self.length = length;
    }
}

fn main() {}

fn try_file(p: impl AsRef<Path>) {
    let mut store = TrackStore::default();
    let trap = TrackTrap::default();

    if let Err(e) = read_tag_from_file(p.as_ref(), &mut store, &trap) {
        trap.0.set(true);
        println!("error: {e}");
    }

    if store.length.is_none() {
        trap.0.set(true);
        println!("info: no length set.");
    }

    if trap.0.get() {
        println!("In file: {}", p.as_ref().display());
    }
}
