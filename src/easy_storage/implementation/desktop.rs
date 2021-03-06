use std::path::PathBuf;
use std::io::{Read, Write};
use std::fs::{create_dir_all, remove_file};
use xdg::BaseDirectories;
use super::super::{FileMode, File, Dir, Storage};

struct FileImpl {
    path: PathBuf
}

impl FileImpl {
    fn new(path: PathBuf) -> Self {
        FileImpl {
            path: path
        }
    }
}

impl File for FileImpl {
    fn exists(&self) -> bool {
        self.path.exists()
    }

    fn remove(&mut self) -> std::io::Result<()> {
        remove_file(self.path.clone())
    }

    fn reader(&self) -> std::io::Result<Box<dyn Read>> {
        Ok(Box::new(std::fs::File::open(self.path.clone())?))
    }

    fn writer(&mut self) -> std::io::Result<Box<dyn Write>> {
        if let Some(dir_path) = self.path.parent() {
            create_dir_all(dir_path)?;
        }
        Ok(Box::new(std::fs::File::create(self.path.clone())?))
    }
}

struct DirImpl {
    path: PathBuf
}

impl DirImpl {
    fn new(path: PathBuf) -> Self {
        DirImpl {
            path: path
        }
    }
}

impl Dir for DirImpl {
    fn file(&self, name: String, _mode: FileMode) -> Box<dyn File> {
        let mut path = self.path.clone();
        path.push(name);
        Box::new(FileImpl::new(path))
    }

    fn subdir(&self, name: String) -> Box<dyn Dir> {
        let mut path = self.path.clone();
        path.push(name);
        Box::new(DirImpl::new(path))
    }
}

struct StorageImpl {
    base_dirs: BaseDirectories
}

impl StorageImpl {
    fn new(name: String) -> Option<Self> {
        BaseDirectories::with_prefix(name).map(|base_dirs| {
            StorageImpl {
                base_dirs: base_dirs
            }
        }).ok()
    }
}

impl Storage for StorageImpl {
    fn data(&mut self) -> Box<dyn Dir> {
        Box::new(DirImpl::new(self.base_dirs.get_data_home()))
    }

    fn config(&mut self) -> Box<dyn Dir> {
        Box::new(DirImpl::new(self.base_dirs.get_config_home()))
    }

    fn cache(&mut self) -> Box<dyn Dir> {
        Box::new(DirImpl::new(self.base_dirs.get_config_home()))
    }
}

pub fn get_storage(name: String) -> Option<Box<dyn Storage>> {
    match StorageImpl::new(name) {
        Some(storage) => Some(Box::new(storage)),
        None => None
    }
}
