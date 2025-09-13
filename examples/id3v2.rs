use std::process::ExitCode;

use ratag::{Result, id3, tag, trap};

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
    let mut tag = tag::Basic::default();
    id3::v2::from_file(
        "/home/kubas/music/Avicii - Taste The Feeling (Avicii Vs. Conrad Sewell).mp3",
        &mut tag,
        &trap::Warn,
    )?;
    println!("{tag:#?}");
    Ok(())
}
