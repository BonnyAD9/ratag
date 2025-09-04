use std::process::ExitCode;

use ratag::{BasicTag, Result, id3, trap};

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
    id3::v2::from_file(
        "/home/kubas/music/4tet - 1st - 01 Addams Family Theme.mp3",
        &mut tag,
        &trap::Skip,
    )?;
    println!("{tag:#?}");
    Ok(())
}
