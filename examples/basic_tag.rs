use ratag::{Result, tag};

fn main() -> Result<()> {
    let tag = tag::Basic::from_file(
        "/home/kubas/music/4tet - 1st - 01 Addams Family Theme.mp3",
    )?;
    println!("{:#?}", tag);

    let tag = tag::Basic::from_file(
        "/home/kubas/music/Jacob Collier - Djesse Vol. 4/14. Box Of Stars - Part 1 (feat. Kirk Franklin, CHIKA, D Smoke, Sho Madjozi, YELLE, Kanyi Mavi).flac",
    )?;
    println!("{:#?}", tag);

    let tag = tag::Basic::from_file(
        "/home/kubas/music/Imagine Dragons - iTunes Session - 01 It's Time.m4a",
    )?;
    println!("{:#?}", tag);

    Ok(())
}
