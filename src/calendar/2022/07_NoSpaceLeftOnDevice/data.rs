use std::{ cell::RefCell, collections::HashMap, rc::Rc };

pub type DirectoryRef = Rc<RefCell<DirectoryEntry>>;
pub type FileRef = Rc<RefCell<FileEntry>>;

pub struct DirectoryEntry {
    pub files: HashMap<String, FileRef>,
    pub directories: HashMap<String, DirectoryRef>
}

impl DirectoryEntry {
    pub fn new() -> DirectoryEntry {
        DirectoryEntry { 
            files: HashMap::<_, _>::new(),
            directories: HashMap::<_, _>::new()
        }
    }

    pub fn new_ref() -> DirectoryRef {
        DirectoryRef::new(RefCell::new(DirectoryEntry::new()))
    }
}

pub struct FileEntry {
    pub size: usize
}

impl FileEntry {
    pub fn new_ref(size: usize) -> FileRef {
        FileRef::new(RefCell::new(FileEntry { size }))
    }
}