use ratag::{BasicTag, Result};

fn main() -> Result<()> {
    let tag = BasicTag::from_file(
        "/home/kubas/music/4tet - 1st - 01 Addams Family Theme.mp3",
    )?;
    println!("{:#?}", tag);
    Ok(())
}
