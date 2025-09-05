use ratag::{BasicTag, Result, mp4, trap};

fn main() -> Result<()> {
    let mut tag = BasicTag::default();
    mp4::from_path(
        "/home/kubas/music/Imagine Dragons - iTunes Session - 01 It's Time.m4a",
        &mut tag,
        &trap::Warn,
    )?;
    println!("{tag:#?}");
    Ok(())
}
