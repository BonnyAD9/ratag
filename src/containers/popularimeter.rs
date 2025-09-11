/// Popularimeter. (rating of user)
#[derive(Debug, Clone)]
pub struct Popularimeter {
    /// Email of the user that rated.
    pub email: String,
    /// Rating given by user.
    pub rating: u8,
    /// Play counter of the user.
    pub play_counter: u64,
}
