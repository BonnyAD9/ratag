use std::{fs::File, io::Write};

use ratag::{PictureTag, Result};

pub fn main() -> Result<()> {
    let tag = PictureTag::read_cover(
        // "/home/kubas/music/4tet - 1st - 02 How Deep Is Your Love.mp3"
        // "/home/kubas/music/AJR - Neotheater - 01 Next Up Forever.flac"
        "/home/kubas/music/Imagine Dragons - iTunes Session - 01 It's Time.m4a",
    )?;
    let Some(pic) = tag.picture() else {
        println!("No picture");
        return Ok(());
    };

    File::create("tmp/pic.jpg")?.write_all(&pic.data)?;
    Ok(())
}
