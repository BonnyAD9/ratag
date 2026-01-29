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
        //"/home/kubas/music/Gotye - Like Drawing Blood - 01 Like Drawing Blood.mp3", // .2
        //"/home/kubas/music/Alan Walker - Different World - 11 Darkside.mp3", // .3
        "/home/kubas/music/ZRNÍ - Voní 01 Dítě vidí psa.mp3", // .4
        //"/home/kubas/music/Avicii - Taste The Feeling (Avicii Vs. Conrad Sewell).mp3",
        &mut tag,
        &trap::Warn,
    )?;
    println!("{tag:#?}");
    Ok(())
}
