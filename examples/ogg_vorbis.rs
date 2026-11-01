use ratag::{Result, ogg, tag, trap};

fn main() -> Result<()> {
    let mut tag = tag::Basic::default();
    let path = "/home/kubas/documents/school/gzn/vorbis/music-vorbis.ogg";
    ogg::from_file(path, &mut tag, &trap::Warn)?;
    println!("{tag:#?}");
    Ok(())
}
