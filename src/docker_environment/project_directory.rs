use std::{
    path::PathBuf,
    fs::{
        exists,
        remove_dir_all,
        create_dir,
    },
    io::Error,
};
use std::io::Write;

use crate::docker_environment::file::CodeFile;


pub struct Directory<'a>(
    pub &'a str,
    pub Option<
            Box<
                [PrjFile<'a>]
            >
        >
);


impl<'a> Directory<'a> {


    fn create_directory(self, curr_folder: PathBuf) -> ()
    {
        let Directory(dir_name, maybe_box_dir_contents) = self;

        let mut new_dir = curr_folder.clone();
        new_dir.push(dir_name);

        let _ = create_dir(&new_dir);

        match maybe_box_dir_contents {

            Some(box_dir_contents) => {

                for prf_file in box_dir_contents {

                    prf_file.create_file_blob(
                        new_dir.clone()
                    );
                };
            },

            None => (),
        }
    }

    fn get_dirname_str(&self) -> String {

        let Directory(dir_name, _): &Directory = self;

        dir_name.to_string()

    }


}


pub enum PrjFile<'a> {
    Dir(Directory<'a>),
    DirFile(CodeFile<'a>),
}


impl<'a> PrjFile<'a> {

    pub fn create_file_blob(self, current_dir: PathBuf) -> () {
        match self {

            PrjFile::Dir(directory) => directory.create_directory(current_dir),
            PrjFile::DirFile(code_file) => code_file.create_file(current_dir),
        }
    }
}


pub struct ProjectDirectory<'a>(pub PathBuf, pub Directory<'a>);




impl<'a> ProjectDirectory<'a> {

    pub fn build(self) -> Result<(), Error>
    {

        eprintln!("change this current dir to an immutable directory, use pointers!!!!");

        let ProjectDirectory(current_path, directory) = self;

        let mut dir_to_be_created = current_path.clone();

        dir_to_be_created.push(directory.get_dirname_str());


        let () = match exists(&dir_to_be_created) {

            Err(err) => return Err(err),

            Ok(true) => remove_dir_all(dir_to_be_created.clone())?,

            Ok(false) => (),
        };

        Ok(
            directory.create_directory(current_path)
        )
    }
}

