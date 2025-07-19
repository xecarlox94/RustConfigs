use std::{fs::File, io::Write, path::PathBuf};

#[derive(Debug)]
pub enum DirFile {
    Doc(Text), // WIP: Create interface to get content str from FileContent
    Exec(Code), // WIP: content should be a pointer
}

pub trait CreateFile {
    fn create_file(&self, current_dir: PathBuf) -> std::io::Result<File>;
}

impl CreateFile for DirFile {
    fn create_file(&self, current_dir: PathBuf) -> std::io::Result<File> {
        match self {
            DirFile::Doc(text_file) => text_file.create_file(current_dir),
            DirFile::Exec(code_file) => code_file.create_file(current_dir),
        }
    }
}

#[derive(Debug)]
pub struct Text(pub String, pub String);

#[derive(Debug)]
pub struct Code(pub Text);

impl CreateFile for Code {
    fn create_file(&self, current_dir: PathBuf) -> std::io::Result<File> {
        self.0.create_file(current_dir).and_then(|written_file| {
            written_file.metadata().map(|file_metata| {
                use std::os::unix::fs::PermissionsExt as _;

                let mut perms = file_metata.permissions();

                perms.set_mode(0o755);

                let v = written_file.set_permissions(perms);

                written_file
            })
        })
    }
}

impl CreateFile for Text {
    fn create_file(&self, current_dir: PathBuf) -> std::io::Result<File> {
        let Text(ref file_name, ref content) = self;

        File::create({
            let mut file_dir = current_dir.clone();
            file_dir.push(file_name);
            file_dir
        })
        .and_then(|mut f| f.write_all(content.as_bytes()).map(|()| f))
    }
}
