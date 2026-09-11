/// Trait for tags that can provide their typical file extensions.
pub trait TagFormat {
    /// Extensions that are usually asociated with this format. This format
    /// will be prioritized for files with one of these extensions.
    fn extensions(&self) -> &[&str];
}
