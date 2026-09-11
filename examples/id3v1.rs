use std::process::ExitCode;

use ratag::{
    Result, WriteMode,
    id3::{self, v1::Id3v1Tag},
    tag, trap,
};

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
    id3::v1::from_file(
        //"/home/kubas/music/Bastille - Goosebumps EP - 03 WHAT YOU GONNA DO (feat. Graham Coxon).mp3",
        "tmp/tag.mp3",
        &mut tag,
        &trap::Warn,
    )?;
    println!("{tag:#?}");
    tag.title = Some("WHAT YOU GONN DO (feat. Graham Coxon)".into());
    //id3::v1::write_to_file(
    //    "tmp/tag2.mp3",
    //    &tag,
    //    WriteMode::update().add(),
    //    &trap::Skip,
    //)?;
    id3::v1::migrate_version_file("tmp/tag2.mp3", false, &trap::Skip, 1)?;
    Ok(())
}
