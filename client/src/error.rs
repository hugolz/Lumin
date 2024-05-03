#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error while using the player {0}")]
    Player(#[from] PlayerError),
    #[error("Error while using the Wallpaper {0}")]
    Wallpaper(#[from] WallpaperError),
}

#[derive(thiserror::Error, Debug)]
pub enum WallpaperError {
    #[error("The search for the player with id ({0:?}) did not find any match")]
    PlayerDontExist(crate::id::ID),
}

#[derive(thiserror::Error, Debug)]
pub enum PlayerError {
    #[error("Io: {0}")]
    Io(#[from] std::io::Error),
    #[error("Verification error: {0}")]
    Verification(String),
}
