use std::{
    fs::{create_dir, exists, remove_dir_all},
    io::Error,
    path::PathBuf,
};


use super::file::{CreateFile as _, DirFile};

#[derive(Debug)]
pub struct Directory(pub String, pub Option<Box<[Blob]>>);

impl Directory {
    fn create_directory(&self, curr_folder: PathBuf) -> std::io::Result<()> {
        let Directory(dir_name, maybe_box_dir_contents) = self;

        let mut new_dir = curr_folder.clone();
        new_dir.push(dir_name);

        create_dir(&new_dir).map_err(|e| e.to_string()); // FIX: handle this error

        maybe_box_dir_contents
            .as_ref()
            .map(|box_dir_contents| {
                box_dir_contents
                    .iter()
                    .map(|p_file| p_file.create_file_blob(new_dir.clone()))
                    .filter_map(|v| match v {
                        Ok(_) => None,
                        Err(e) => Some(e), // FIX: fix error handling in this region
                    })
                    .collect::<Vec<_>>()
            });

        Ok(())
    }

    fn get_dirname_str(&self) -> String {
        let Directory(dir_name, _) = self;

        dir_name.to_string()
    }
}

#[derive(Debug)]
pub enum Blob {
    Branch(Directory),
    Leaf(DirFile)
}

impl Blob {
    pub fn create_file_blob(&self, current_dir: PathBuf) -> std::io::Result<()> {
        match self {
            Blob::Branch(directory) => directory.create_directory(current_dir),
            Blob::Leaf(file_prj) => file_prj.create_file(current_dir).map(|_| ()),
        }
    }
}

pub struct ProjectDirectory(pub PathBuf, pub Directory);

impl ProjectDirectory {
    pub fn build(self) -> std::io::Result<()> {
        // eprintln!("change this current dir to an immutable directory, use pointers!!!!");

        let ProjectDirectory(current_path, directory) = self;

        let mut dir_to_be_created = current_path.clone();

        dir_to_be_created.push(directory.get_dirname_str());

        let () = match exists(&dir_to_be_created) {
            Err(err) => return Err(err),

            Ok(true) => remove_dir_all(dir_to_be_created.clone())?,

            Ok(false) => (),
        };

        directory.create_directory(current_path)
    }
}
