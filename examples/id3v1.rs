use std::process::ExitCode;

use ratag::{Result, id3::Id3v1};

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
    let tag = Id3v1::from_file(
        "/home/kubas/music/4tet - 1st - 01 Addams Family Theme.mp3",
    )?;
    println!("{tag:#?}");
    Ok(())
}
