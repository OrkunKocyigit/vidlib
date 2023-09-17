use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct VideoCategory {
    id: usize,
    name: String,
    icon: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct VideoEntry {
    name: String,
    rating: usize,
    notes: String,
    watched: bool,
    category: Option<VideoCategory>,
}

impl VideoEntry {
    pub fn new(name: String, rating: usize, notes: String, watched: bool) -> Self {
        Self {
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
    pub fn set_rating(&mut self, rating: usize) {
        self.rating = rating;
    }
    pub fn set_watched(&mut self, watched: bool) {
        self.watched = watched;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_notes(&mut self, notes: String) {
        self.notes = notes;
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
