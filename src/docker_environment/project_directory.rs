use std::{
    path::PathBuf,
    fs::{
        exists,
        remove_dir_all,
    },
    io::Error,
};

pub use crate::docker_environment::file::{CodeFile};


pub struct Directory<'a>(
    pub &'a str,
    pub Option<
            Box<
                [PrjFile<'a>]
            >
        >
);


pub enum PrjFile<'a> {
    Dir(Directory<'a>),
    DirFile(CodeFile<'a>),
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

