use std::path::Path;

use crate::{
    DataType, Picture as Pic, PictureKind, Result, TagStore,
    read_tag_from_file, trap,
};

/// Tag that reads picture from the file.
#[derive(Debug)]
pub struct Picture {
    /// Accepted types of picture.
    pub types: PictureKind,
    /// Maximum number of pictures to load.
    pub max_cnt: usize,
    /// Precedence of picture kinds. First has higher precedence.
    pub precedence: Vec<PictureKind>,
    /// Loaded pictures.
    pub pictures: Vec<Pic>,
}

impl TagStore for Picture {
    fn stores_data(&self, typ: DataType) -> bool {
        let DataType::Picture(pk) = typ else {
            return false;
        };
        if !pk.intersects(self.types) {
            return false;
        }
        if self.pictures.len() < self.max_cnt {
            return true;
        }
        self.find_low_prec(pk).is_some()
    }

    fn add_picture(&mut self, picture: Pic) {
        if !picture.kind.intersects(self.types) {
            return;
        }

        if self.pictures.len() < self.max_cnt {
            self.pictures.push(picture);
            return;
        }

        if let Some(idx) = self.find_low_prec(picture.kind) {
            self.pictures[idx] = picture;
        }
    }

    fn done(&self) -> bool {
        self.pictures.len() >= self.max_cnt
            && (self.precedence.is_empty()
                || self.pictures.iter().all(|p| p.kind == self.precedence[0]))
    }
}

impl Picture {
    /// Gets configuration that will load any picture, but it will try to get
    /// the best cover.
    pub fn preferably_cover() -> Self {
        Self {
            types: PictureKind::all(),
            max_cnt: 1,
            precedence: vec![
                PictureKind::FRONT_COVER,
                PictureKind::BACK_COVER,
                PictureKind::OTHER,
            ],
            pictures: vec![],
        }
    }

    /// Load any picture, but it will try to get the best cover.
    pub fn read_cover(p: impl AsRef<Path>) -> Result<Self> {
        let mut res = Self::preferably_cover();
        read_tag_from_file(p, &mut res, &trap::Skip)?;
        Ok(res)
    }

    /// Get the first best picture.
    pub fn picture(&self) -> Option<&Pic> {
        self.pictures
            .iter()
            .min_by_key(|p| self.get_precedence(p.kind).unwrap_or(usize::MAX))
    }

    fn get_precedence(&self, pk: PictureKind) -> Option<usize> {
        self.precedence.iter().position(|a| *a == pk)
    }

    fn find_low_prec(&self, pk: PictureKind) -> Option<usize> {
        let prec = self.get_precedence(pk)?;
        self.pictures
            .iter()
            .map(|a| self.get_precedence(a.kind).unwrap_or(usize::MAX))
            .enumerate()
            .max()
            .filter(|(_, p)| *p > prec)
            .map(|(i, _)| i)
    }
}
