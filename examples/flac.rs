use ratag::{Result, flac, tag, trap};

fn main() -> Result<()> {
    let mut tag = tag::Basic::default();
    let path = "/home/kubas/music/The Greatest Showman - 05 Hugh Jackman - The Other Side (With Zac Efron).flac";
    flac::from_file(path, &mut tag, &trap::Warn)?;
    println!("{tag:#?}");
    Ok(())
}
