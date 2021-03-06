use std::io::{Read, Write};
use web_sys::Storage as WebStorage;
use super::super::{FileMode, File, Dir, Storage};

mod text_values;
mod binary_values;

use text_values::{TextStorageReader, TextStorageWriter};
use binary_values::{BinaryStorageReader, BinaryStorageWriter};

struct FileImpl {
    web_storage: WebStorage,
    path: String,
    mode: FileMode
}

impl FileImpl {
    fn new(web_storage: &WebStorage, path: String, mode: FileMode) -> Self {
        FileImpl {
            web_storage: web_storage.clone(),
            path: path,
            mode: mode
        }
    }
}

impl File for FileImpl {
    fn exists(&self) -> bool {
        let value = self.web_storage.get_item(&self.path).unwrap();
        value.is_some()
    }

    fn remove(&mut self) -> std::io::Result<()> {
        self.web_storage.remove_item(&self.path)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "cannot remove value"))
    }

    fn reader(&self) -> std::io::Result<Box<dyn Read>> {
        match self.mode {
            FileMode::Text => Ok(Box::new(TextStorageReader::new(&self.web_storage, &self.path)?)),
            FileMode::Binary => Ok(Box::new(BinaryStorageReader::new(&self.web_storage, &self.path)?))
        }
    }

    fn writer(&mut self) -> std::io::Result<Box<dyn Write>> {
        match self.mode {
            FileMode::Text => Ok(Box::new(TextStorageWriter::new(&self.web_storage, &self.path)?)),
            FileMode::Binary => Ok(Box::new(BinaryStorageWriter::new(&self.web_storage, &self.path)?))
        }
    }
}

struct DirImpl {
    web_storage: WebStorage,
    path: String
}

impl DirImpl {
    fn new(web_storage: &WebStorage, path: String) -> Self {
        DirImpl {
            web_storage: web_storage.clone(),
            path: path
        }
    }
}

impl Dir for DirImpl {
    fn file(&self, name: String, mode: FileMode) -> Box<dyn File> {
        let mut path = self.path.clone();
        path.push_str(&name);
        Box::new(FileImpl::new(&self.web_storage, path, mode))
    }

    fn subdir(&self, name: String) -> Box<dyn Dir> {
        let mut path = self.path.clone();
        path.push_str(&name);
        Box::new(DirImpl::new(&self.web_storage, path))
    }
}

struct StorageImpl {
    web_storage: WebStorage
}

impl StorageImpl {
    fn new(_name: String) -> Option<Self> {
        let web_storage = web_sys::window().unwrap().local_storage().unwrap();
        if let Some(web_storage) = web_storage {
            let storage = StorageImpl {
                web_storage: web_storage
            };
            Some(storage)
        } else {
            None
        }
    }

    fn dir(&self, name: &str) -> Box<dyn Dir> {
        Box::new(DirImpl::new(&self.web_storage, name.to_string()))
    }
}

impl Storage for StorageImpl {
    fn data(&mut self) -> Box<dyn Dir> {
        self.dir("data")
    }

    fn config(&mut self) -> Box<dyn Dir> {
        self.dir("config")
    }

    fn cache(&mut self) -> Box<dyn Dir> {
        self.dir("cache")
    }
}

pub fn get_storage(name: String) -> Option<Box<dyn Storage>> {
    match StorageImpl::new(name) {
        Some(storage) => Some(Box::new(storage)),
        None => None
    }
}
