//use std::fs;

pub struct SubFolder {
    pub name: String,
    pub selected: bool
}

pub struct IgnoreFile {
    _path: String,
    pub folders: Vec<SubFolder>
}

impl IgnoreFile {
    pub fn find(_path: &str) -> Result<String, String> {
        todo!()
    }

    pub fn open(_filename: &str) -> Result<Self, String> {
        todo!()
    }

    pub fn set(&mut self, _sub_folder: &str, _value: bool) -> Result<(), String> {
        todo!()
    }

    pub fn save(&self) -> Result<(), String> {
        todo!()
    }
}