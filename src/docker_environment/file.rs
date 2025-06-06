use std::{fs::File, io::Write, path::PathBuf};

pub enum FilePrj<'a, 'b> {
    Text(TextFile<'a, 'b>),
    Code(CodeFile<'a, 'b>),
}

pub trait CreateFile {
    fn create_file(&self, current_dir: PathBuf) -> std::io::Result<File>;
}

impl<'a, 'b> CreateFile for FilePrj<'a, 'b> {
    fn create_file(&self, current_dir: PathBuf) -> std::io::Result<File> {
        self.create_file(current_dir)
    }
}

pub struct TextFile<'a, 'b>(pub &'a str, pub &'b str);

pub struct CodeFile<'a, 'b>(pub TextFile<'a, 'b>);

impl<'a, 'b> CreateFile for CodeFile<'a, 'b> {
    fn create_file(&self, current_dir: PathBuf) -> std::io::Result<File> {
        self.create_file(current_dir).and_then(|written_file| {
            written_file.metadata().map(|file_metata| {
                use std::os::unix::fs::PermissionsExt as _;

                // TODO: verify if this is correct!!!

                file_metata.permissions().set_mode(755u32);

                written_file
            })
        })
    }
}

impl<'a, 'b> CreateFile for TextFile<'a, 'b> {
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
