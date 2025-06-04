use std::{
    fs::File, io::Write, os::unix::fs::PermissionsExt as _, path::PathBuf
};


pub struct CodeFile<'a>(pub &'a str, pub String, pub bool);


impl<'a> CodeFile<'a> {

    pub fn create_file(self, current_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {

        let CodeFile(file_name, content, exec) = self;

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

                let file_metata = docker_file.metadata()?;

                file_metata.permissions().set_mode(755u32);
            },
            Err(_) => (),
        };

        Ok(())
    }
}

