use std::{fs::File, io::BufReader, process::ExitCode};

use ratag::{BasicTag, Result, id3::read_id3v2, trap};

fn main() -> ExitCode {
    match start() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn start() -> Result<()> {
    let mut tag = BasicTag::default();
    let f = BufReader::new(File::open(
        "/home/kubas/music/4tet - 1st - 01 Addams Family Theme.mp3",
    )?);
    read_id3v2(f, &mut tag, &trap::Skip)?;
    println!("{tag:#?}");
    Ok(())
}
