use ratag::{BasicTag, Result, flac, trap};

fn main() -> Result<()> {
    let mut tag = BasicTag::default();
    let path = "/home/kubas/music/Jacob Collier - Djesse Vol. 4/14. Box Of Stars - Part 1 (feat. Kirk Franklin, CHIKA, D Smoke, Sho Madjozi, YELLE, Kanyi Mavi).flac";
    flac::from_file(path, &mut tag, &trap::Skip)?;
    println!("{tag:#?}");
    Ok(())
}
