use std::{
    fs::{create_dir, exists, remove_dir_all},
    io::Error,
    path::PathBuf,
};
use nonempty::NonEmpty;

use crate::docker_environment::file::FilePrj;

use super::file::CreateFile as _;

#[derive(Debug)]
pub struct Directory(pub String, pub Option<Box<[PrjFile]>>);

impl Directory {
    fn create_directory(self, curr_folder: PathBuf) -> Result<(), String> {
        let Directory(dir_name, maybe_box_dir_contents) = self;

        let mut new_dir = curr_folder.clone();
        new_dir.push(dir_name);

        create_dir(&new_dir).map_err(|e| e.to_string()).and(
            match maybe_box_dir_contents
                .ok_or(Err(String::from("Directory")))
                .map(|box_dir_contents| {
                    NonEmpty::from_vec(
                    box_dir_contents
                        .into_iter()
                        .map(|p_file| p_file.create_file_blob(new_dir.clone()))
                        .filter_map(|v| match v {
                            Ok(_) => None,
                            Err(e) => Some(e),
                        })
                        .collect()
                    )
                    .ok_or(Err(String::from("Directory")))

                    errors
                        .into_iter()
                        .map::<String, fn(_) -> String>(|e| e.to_string())
                        .reduce(|acc_msg: String, item_msg| {
                            acc_msg + "\nError: " + item_msg.as_str()
                        })
                }) {
                    Ok(ok) => ok,
                    Ok(ok) => ok,
                    Err(err) => Err(err),
                },
        )
    }

    fn get_dirname_str(&self) -> String {
        let Directory(dir_name, _): &Directory = self;

        dir_name.to_string()
    }
}

#[derive(Debug)]
pub enum PrjFile {
    Dir(Directory),
    DirFile(FilePrj),
}

impl PrjFile {
    pub fn create_file_blob(self, current_dir: PathBuf) -> std::io::Result<()> {
        match self {
            PrjFile::Dir(directory) => Ok(directory.create_directory(current_dir)),
            PrjFile::DirFile(file_prj) => file_prj.create_file(current_dir).map(|_| ()),
        }
    }
}

pub struct ProjectDirectory(pub PathBuf, pub Directory);

impl ProjectDirectory {
    pub fn build(self) -> Result<(), Error> {
        eprintln!("change this current dir to an immutable directory, use pointers!!!!");

        let ProjectDirectory(current_path, directory) = self;

        let mut dir_to_be_created = current_path.clone();

        dir_to_be_created.push(directory.get_dirname_str());

        let () = match exists(&dir_to_be_created) {
            Err(err) => return Err(err),

            Ok(true) => remove_dir_all(dir_to_be_created.clone())?,

            Ok(false) => (),
        };

        Ok(directory.create_directory(current_path))
    }
}
