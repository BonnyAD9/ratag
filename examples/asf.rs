use ratag::{Result, asf, tag, trap};

fn main() -> Result<()> {
    let mut tag = tag::Basic::default();
    asf::from_file(
        "/home/kubas/music/Luděk Koutný - The Golden Western Melodies - 01 Harmonica.wma",
        &mut tag,
        &trap::Warn,
    )?;
    println!("{tag:#?}");
    Ok(())
}
