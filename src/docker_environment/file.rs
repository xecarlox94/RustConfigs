use std::{fs::File, io::Write, path::PathBuf};

#[derive(Debug)]
pub enum FilePrj {
    Text(TextFile),
    Code(CodeFile),
}

pub trait CreateFile {
    fn create_file(&self, current_dir: PathBuf) -> std::io::Result<File>;
}

impl CreateFile for FilePrj {
    fn create_file(&self, current_dir: PathBuf) -> std::io::Result<File> {
        match self {
            FilePrj::Text(text_file) => text_file.create_file(current_dir),
            FilePrj::Code(code_file) => code_file.create_file(current_dir),
        }
    }
}

#[derive(Debug)]
pub struct TextFile(pub String, pub String);

#[derive(Debug)]
pub struct CodeFile(pub TextFile);

impl CreateFile for CodeFile {
    fn create_file(&self, current_dir: PathBuf) -> std::io::Result<File> {
        self.0.create_file(current_dir).and_then(|written_file| {
            written_file.metadata().map(|file_metata| {
                use std::os::unix::fs::PermissionsExt as _;

                // TODO: verify if this is correct!!!
                // file_metata.permissions().set_mode(755u32);

                written_file
            })
        })
    }
}

impl CreateFile for TextFile {
    fn create_file(&self, current_dir: PathBuf) -> std::io::Result<File> {
        let TextFile(ref file_name, ref content) = self;

        File::create({
            let mut file_dir = current_dir.clone();
            file_dir.push(file_name);
            file_dir
        })
        .and_then(|mut f| f.write_all(content.as_bytes()).map(|()| f))
    }
}
