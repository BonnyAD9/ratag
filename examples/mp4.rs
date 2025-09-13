use ratag::{Result, mp4, tag, trap};

fn main() -> Result<()> {
    let mut tag = tag::Basic::default();
    mp4::from_path(
        "/home/kubas/music/Imagine Dragons - iTunes Session - 01 It's Time.m4a",
        &mut tag,
        &trap::Warn,
    )?;
    println!("{tag:#?}");
    Ok(())
}
