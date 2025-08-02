use std::{
    borrow::Cow, fs::{create_dir, exists, remove_dir_all}, path::PathBuf
};

use super::file::DirFile;

#[derive(Debug)]
pub struct Directory<'a>{
    pub directory_name: &'a str,
    pub contents: Box<[Blob<'a>]>
}

impl<'a> Directory<'a> {
    fn create_directory(&self, curr_folder: PathBuf) -> std::io::Result<()> {
        let Directory {
            directory_name,
            contents
        }= self;

        let mut new_dir = curr_folder.clone();
        new_dir.push(directory_name);

        create_dir(&new_dir).map_err(|e| e.to_string()); // FIX: handle this error

        contents
            .iter()
            .map(|p_file| p_file.create_file_blob(new_dir.clone()))
            .filter_map(|v| v.err())
            .collect::<Vec<_>>();

        Ok(())
    }

    fn get_dirname_str(&self) -> &str {
        &self.directory_name
    }
}

#[derive(Debug)]
pub enum Blob<'a> {
    Branch(Directory<'a>),
    Leaf(DirFile<'a>),
}

impl<'a> Blob<'a> {
    pub fn create_file_blob(&self, current_dir: PathBuf) -> std::io::Result<()> {
        match self {
            Blob::Branch(directory) => directory.create_directory(current_dir),
            Blob::Leaf(file_prj) => file_prj.write_file(current_dir).map(|_| ()),
        }
    }
}

pub struct ProjectDirectory<'a> {
    pub path: Cow<PathBuf>,
    pub dir: Directory<'a>
}

impl<'a> ProjectDirectory<'a> {
    pub fn build(self) -> std::io::Result<()> {
        // eprintln!("change this current dir to an immutable directory, use pointers!!!!");

        let ProjectDirectory {
            path,
            dir
        } = self;

        let mut dir_to_be_created = path.clone();

        dir_to_be_created.push(directory.get_dirname_str());

        let () = match exists(&dir_to_be_created) {
            Err(err) => return Err(err),

            Ok(true) => remove_dir_all(dir_to_be_created.clone())?,

            Ok(false) => (),
        };

        directory.create_directory(current_path)
    }
}
