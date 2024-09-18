use std::collections::HashMap;

pub struct File {
    content: Vec<u8>,
}

pub struct Directory {
    entries: HashMap<String, FileSystemEntry>,
}

pub enum FileSystemEntry {
    File(File),
    Directory(Directory),
}

pub struct FileSystem {
    root: Directory,
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            root: Directory { entries: HashMap::new() },
        }
    }

    pub fn create_file(&mut self, path: &str, content: Vec<u8>) -> Result<(), String> {
        let (parent_path, file_name) = self.split_path(path);
        let parent = self.navigate_to_directory(parent_path)?;
        
        parent.entries.insert(file_name.to_string(), FileSystemEntry::File(File { content }));
        Ok(())
    }

    pub fn create_directory(&mut self, path: &str) -> Result<(), String> {
        let (parent_path, dir_name) = self.split_path(path);
        let parent = self.navigate_to_directory(parent_path)?;
        
        parent.entries.insert(dir_name.to_string(), FileSystemEntry::Directory(Directory { entries: HashMap::new() }));
        Ok(())
    }

    pub fn read_file(&self, path: &str) -> Result<&Vec<u8>, String> {
        let (parent_path, file_name) = self.split_path(path);
        let parent = self.navigate_to_directory(parent_path)?;
        
        match parent.entries.get(file_name) {
            Some(FileSystemEntry::File(file)) => Ok(&file.content),
            _ => Err(format!("File not found: {}", path)),
        }
    }

    fn navigate_to_directory(&self, path: &str) -> Result<&Directory, String> {
        let mut current = &self.root;
        for component in path.split('/').filter(|&c| !c.is_empty()) {
            match current.entries.get(component) {
                Some(FileSystemEntry::Directory(dir)) => current = dir,
                _ => return Err(format!("Directory not found: {}", path)),
            }
        }
        Ok(current)
    }

    fn split_path(&self, path: &str) -> (&str, &str) {
        match path.rfind('/') {
            Some(index) => (&path[..index], &path[index + 1..]),
            None => ("", path),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_system() {
        let mut fs = FileSystem::new();

        fs.create_directory("/home").unwrap();
        fs.create_directory("/home/user").unwrap();
        fs.create_file("/home/user/test.txt", b"Hello, world!".to_vec()).unwrap();

        assert!(fs.create_directory("/home/user/documents").is_ok());
        assert!(fs.create_file("/home/user/documents/doc.txt", b"Document content".to_vec()).is_ok());

        assert_eq!(fs.read_file("/home/user/test.txt").unwrap(), b"Hello, world!");
        assert_eq!(fs.read_file("/home/user/documents/doc.txt").unwrap(), b"Document content");

        assert!(fs.read_file("/nonexistent/file.txt").is_err());
        assert!(fs.create_file("/nonexistent/dir/file.txt", vec![]).is_err());
    }
}