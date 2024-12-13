use std::path::Path;

use tantivy::directory::{
    Directory,
    FileHandle,
    WatchCallback,
    WatchHandle,
};
use tantivy::directory::error::{
    OpenReadError,
    OpenWriteError,
    DeleteError,
};

pub struct IDBDirectory;

impl Directory for IDBDirectory {
    fn get_file_handle(&self, path: &Path) -> Result<Box<dyn FileHandle>, OpenReadError> {
        unimplemented!()
    }

    fn delete(&self, path: &Path) -> Result<(), DeleteError> {
        unimplemented!()
    }

    fn exists(&self, path: &Path) -> Result<bool, OpenReadError> {
        unimplemented!()
    }

    fn open_write(&self, path: &Path) -> Result<FileHandle, OpenWriteError> {
        unimplemented!()
    }

    fn atomic_read(&self, path: &Path) -> Result<Vec<u8>, OpenReadError> {
        unimplemented!()
    }

    fn atomic_write(&self, path: &Path, data: &[u8]) -> std::io::Result<()> {
        unimplemented!()
    }

    fn sync_directory(&self) -> std::io::Result<()> {
        unimplemented!()
    }

    fn watch(&self, _watch_callback: WatchCallback) -> tantivy::Result<WatchHandle> {
        unimplemented!()
    }
}

pub struct OPFSDirectory;

impl Directory for OPFSDirectory {
    fn get_file_handle(&self, path: &Path) -> Result<Box<dyn FileHandle>, OpenReadError> {
        unimplemented!()
    }

    fn delete(&self, path: &Path) -> Result<(), DeleteError> {
        unimplemented!()
    }

    fn exists(&self, path: &Path) -> Result<bool, OpenReadError> {
        unimplemented!()
    }

    fn open_write(&self, path: &Path) -> Result<FileHandle, OpenWriteError> {
        unimplemented!()
    }

    fn atomic_read(&self, path: &Path) -> Result<Vec<u8>, OpenReadError> {
        unimplemented!()
    }

    fn atomic_write(&self, path: &Path, data: &[u8]) -> std::io::Result<()> {
        unimplemented!()
    }

    fn sync_directory(&self) -> std::io::Result<()> {
        unimplemented!()
    }

    fn watch(&self, _watch_callback: WatchCallback) -> tantivy::Result<WatchHandle> {
        unimplemented!()
    }
}
