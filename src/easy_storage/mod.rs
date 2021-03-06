use std::io::{Read, Write};

mod implementation;
pub use implementation::get_storage;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FileMode {
    Text,
    Binary
}

pub trait File {
    fn exists(&self) -> bool;
    fn remove(&mut self) -> std::io::Result<()>;
    fn reader(&self) -> std::io::Result<Box<dyn Read>>;
    fn writer(&mut self) -> std::io::Result<Box<dyn Write>>;
}

pub trait Dir {
    fn file(&self, name: String, mode: FileMode) -> Box<dyn File>;
    fn subdir(&self, name: String) -> Box<dyn Dir>;
}

pub trait Storage {
    fn data(&mut self) -> Box<dyn Dir>;
    fn config(&mut self) -> Box<dyn Dir>;
    fn cache(&mut self) -> Box<dyn Dir>;
}
