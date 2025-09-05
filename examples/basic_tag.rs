use ratag::{BasicTag, Result};

fn main() -> Result<()> {
    let tag = BasicTag::from_file(
        "/home/kubas/music/4tet - 1st - 01 Addams Family Theme.mp3",
    )?;
    println!("{:#?}", tag);

    let tag = BasicTag::from_file(
        "/home/kubas/music/Jacob Collier - Djesse Vol. 4/14. Box Of Stars - Part 1 (feat. Kirk Franklin, CHIKA, D Smoke, Sho Madjozi, YELLE, Kanyi Mavi).flac",
    )?;
    println!("{:#?}", tag);

    Ok(())
}
