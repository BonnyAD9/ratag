/// Determines how the new data interacts with the old data of a tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WriteModeKind {
    /// Write only new data for fields that were unset.
    Fill,
    /// Write all set fields. Leave the other fields.
    Update,
    /// Write only the set fields.
    Overwrite,
}

/// Determines how tag is written.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WriteMode {
    /// How new data interacts with the old data.
    pub kind: WriteModeKind,
    /// Whether to write new tag if there was no old tag.
    /// - `true` - tag is written even if there previously was no tag.
    /// - `false` - [`crate::Error::NoTag`] is returned if there is no tag.
    pub add: bool,
}

impl WriteMode {
    /// Create new write mode.
    pub fn new(kind: WriteModeKind, add: bool) -> Self {
        Self { kind, add }
    }

    /// Create new filling write mode. Disallow adding new tag.
    pub fn fill() -> Self {
        WriteModeKind::Fill.into()
    }

    /// Create new updateing write mode. Disallow adding new tag.
    pub fn update() -> Self {
        WriteModeKind::Update.into()
    }

    /// Create new overwriting write mode. Disallow adding enw tag.
    pub fn overwrite() -> Self {
        WriteModeKind::Overwrite.into()
    }

    /// Allow adding new tag.
    pub fn add(self) -> Self {
        self.with_add(true)
    }

    /// Set the value of `add`. It determines whether new tag is writen when
    /// there was previously no tag.
    pub fn with_add(mut self, add: bool) -> Self {
        self.add = add;
        self
    }
}

impl From<WriteModeKind> for WriteMode {
    fn from(value: WriteModeKind) -> Self {
        Self::new(value, false)
    }
}

impl PartialEq<WriteModeKind> for WriteMode {
    fn eq(&self, other: &WriteModeKind) -> bool {
        self.kind == *other
    }
}
