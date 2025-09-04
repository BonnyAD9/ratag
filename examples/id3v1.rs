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
        "/home/kubas/music/4tet - 1st - 01 Addams Family Theme.mp3",
        &trap::Skip,
    )?;
    println!("{tag:#?}");
    Ok(())
}
