use std::sync::Mutex;

use crate::video::VideoEntry;
use rusqlite::Connection;

pub struct AppState {
    pub db: Mutex<Option<Connection>>,
    pub videos: Mutex<Option<Vec<VideoEntry>>>,
}
