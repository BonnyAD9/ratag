/// Comment for song.
#[derive(Debug)]
pub struct Comment {
    /// Language of the comment as ISO 639-1 code.
    pub language: Option<String>,
    /// Description of the comment.
    pub desciption: Option<String>,
    /// Value of the comment itself.
    pub value: String,
}

impl Comment {
    /// Create coment with only the value.
    pub fn from_value(value: String) -> Self {
        Self {
            language: None,
            desciption: None,
            value,
        }
    }
}
