use ratag::{BasicTag, Result, riff, trap};

fn main() -> Result<()> {
    let mut tag = BasicTag::default();
    riff::from_file("/home/kubas/test/test.wav", &mut tag, &trap::Warn)?;
    println!("{tag:#?}");
    Ok(())
}
