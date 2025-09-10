use ratag::{BasicTag, Result, asf, trap};

fn main() -> Result<()> {
    let mut tag = BasicTag::default();
    asf::from_file(
        "/home/kubas/music/Luděk Koutný - The Golden Western Melodies - 01 Harmonica.wma",
        &mut tag,
        &trap::Warn,
    )?;
    println!("{tag:#?}");
    Ok(())
}
