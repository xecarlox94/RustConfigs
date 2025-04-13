use std::{
    fs::File,
    path::PathBuf,
    io::Write,
};

pub struct CodeFile<'a>(pub &'a str, pub String);


impl<'a> CodeFile<'a> {

    pub fn create_file(self, current_dir: PathBuf) -> () {

        let CodeFile(file_name, content) = self;

        let docker_file_path = {
            let mut file_dir = current_dir.clone();
            file_dir.push(file_name);
            file_dir
        };

        match File::create(docker_file_path) {

            Ok(mut docker_file) => {
                let _ = docker_file.write_all(
                    content.as_bytes()
                );
            },
            Err(_) => (),
        };

    }
}

