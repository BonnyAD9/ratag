use std::{
    cell::Cell, env::args, fs::read_dir, path::Path, rc::Rc, time::Duration,
};

use encoding::DecoderTrap;
use ratag::{Error, Result, TagStore, read_tag_from_file, trap::Trap};

const EXTENSIONS: &[&str] = &[
    "mp3", "mpga", "bit", "flac", "mp4", "m4a", "m4p", "m4b", "m4r", "m4v",
    "asf", "wma", "wmv",
];

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
    title: Option<String>,
}

impl TagStore for TrackStore {
    fn stores_data(&self, _typ: ratag::DataType) -> bool {
        true
    }

    fn set_length(&mut self, length: Duration) {
        self.length = Some(length);
    }

    fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }
}

fn main() -> Result<()> {
    for a in args().skip(1) {
        try_path(a)?;
    }
    Ok(())
}

fn try_path(p: impl AsRef<Path>) -> Result<()> {
    if p.as_ref().is_dir() {
        try_dir(p)
    } else {
        try_file(p);
        Ok(())
    }
}

fn try_dir(p: impl AsRef<Path>) -> Result<()> {
    for i in read_dir(p)? {
        let p = i?.path();
        if p.is_dir() {
            try_dir(p)?;
        } else if let Some(ext) = p.extension()
            && EXTENSIONS.iter().any(|e| ext == *e)
        {
            try_file(p);
        }
    }

    Ok(())
}

fn try_file(p: impl AsRef<Path>) {
    let mut store = TrackStore::default();
    let trap = TrackTrap::default();
    //eprintln!("FILE: {}", p.as_ref().display());

    if let Err(e) = read_tag_from_file(p.as_ref(), &mut store, &trap) {
        //if !matches!(e, Error::Unsupported(_)) {
        trap.0.set(true);
        println!("error: {e}");
        //}
    } else {
        if store.title.is_none() {
            trap.0.set(true);
            println!("info: no length and title set.");
        }
    }

    if trap.0.get() {
        println!("^IN FILE: {}", p.as_ref().display());
        println!("{:->80}", "")
    }
}
