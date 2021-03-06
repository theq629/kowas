use std::io::{Read, Write, Cursor};
use web_sys::Storage as WebStorage;

pub struct BinaryStorageReader {
    cursor: Cursor<Vec<u8>>
}

impl BinaryStorageReader {
    pub fn new(web_storage: &WebStorage, key: &str) -> std::io::Result<Self> {
        let value = web_storage.get_item(key).unwrap();
        if let Some(value) = value {
            let value = value.chars()
                .map(|c| c as u8)
                .collect();
            let new = BinaryStorageReader {
                cursor: Cursor::new(value)
            };
            Ok(new)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "cannot find stored value"))
        }
    }
}

impl Read for BinaryStorageReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.cursor.read(buf)
    }
}

pub struct BinaryStorageWriter {
    web_storage: WebStorage,
    key: String,
    buf: Vec<u8>

}

impl BinaryStorageWriter {
    pub fn new(web_storage: &WebStorage, key: &str) -> std::io::Result<Self> {
        let new = BinaryStorageWriter {
            web_storage: web_storage.clone(),
            key: key.to_string(),
            buf: Vec::new()
        };
        Ok(new)
    }
}

impl Write for BinaryStorageWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let value = self.buf.iter()
            .map(|b| *b as char)
            .collect::<String>();
        self.web_storage.set_item(&self.key, &value)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "cannot store value"))
    }
}

impl Drop for BinaryStorageWriter {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}
