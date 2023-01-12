use std::fs;

pub struct SubFolder {
    pub name: String,
    pub selected: bool
}

pub struct IgnoreFile {
    path: String,
    pub folders: Vec<SubFolder>
}

impl IgnoreFile {
    pub fn find(path: &str) -> Result<String, String> {

    }

    pub fn open(filename: &str) -> Result<Self, String> {

    }

    pub fn set(&mut self, sub_folder: &str, value: bool) -> Result<(), String> {

    }

    pub fn save(&self) -> Result<(), String> {

    }
}