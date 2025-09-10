use crate::Popularimeter;

#[derive(Debug, Clone)]
pub enum Rating {
    Text(String),
    Popularimeter(Popularimeter),
}
