use std::process::ExitCode;

use ratag::{Result, id3::v1::Id3v1Tag, trap};

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
    let tag = Id3v1Tag::from_file(
        "/home/kubas/music/Bastille - Goosebumps EP - 03 WHAT YOU GONNA DO (feat. Graham Coxon).mp3",
        &trap::Warn,
    )?;
    println!("{tag:#?}");
    Ok(())
}
