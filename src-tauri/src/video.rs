use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Deserialize, Serialize, Clone)]
pub struct VideoCategory {
    id: usize,
    name: String,
    icon: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct VideoEntry {
    pub id: String,
    name: String,
    rating: usize,
    notes: String,
    watched: bool,
    category: Option<VideoCategory>,
}

impl VideoEntry {
    pub fn new(id: String, name: String, rating: usize, notes: String, watched: bool) -> Self {
        Self {
            id,
            name,
            rating,
            notes,
            watched,
            category: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn rating(&self) -> usize {
        self.rating
    }
    pub fn notes(&self) -> &str {
        &self.notes
    }
    pub fn watched(&self) -> bool {
        self.watched
    }
}

pub fn is_video<P: AsRef<Path>>(path: P) -> bool {
    let mut file = match File::open(path) {
        Ok(f) => f,
        _ => return false,
    };
    let mut buf = vec![0; 1024];
    let result = file.read_exact(&mut buf);
    if result.is_err() {
        false
    } else {
        infer::is_video(&*buf)
    }
}
