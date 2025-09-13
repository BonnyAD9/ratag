use crate::{DataType, TagStore, TagType};

/// Tag that reads only the tag types.
pub struct Probe {
    /// Detected tag types.
    pub tags: Vec<TagType>,
    /// If false, tag reading will usually only detect the primary tag and
    /// exit.
    pub thorough: bool,
}

impl Probe {
    /// Detect only the primary tag. This is fast.
    pub fn top_level() -> Self {
        Self {
            tags: vec![],
            thorough: false,
        }
    }

    /// Detect all tags. This is slow.
    pub fn thorough() -> Self {
        Self {
            tags: vec![],
            thorough: true,
        }
    }
}

impl TagStore for Probe {
    fn done(&self) -> bool {
        self.thorough || !self.tags.is_empty()
    }

    fn stores_data(&self, typ: DataType) -> bool {
        typ == DataType::TagType
    }

    fn set_tag_type(&mut self, tag: TagType) {
        self.tags.push(tag);
    }
}
