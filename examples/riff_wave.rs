use ratag::{Result, riff, tag, trap};

fn main() -> Result<()> {
    let mut tag = tag::Basic::default();
    riff::from_file("/home/kubas/test/test.wav", &mut tag, &trap::Warn)?;
    println!("{tag:#?}");
    Ok(())
}
