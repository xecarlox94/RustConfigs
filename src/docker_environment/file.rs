use std::{borrow::Cow, fs::File, io::Write, path::PathBuf};

#[derive(Debug)]
pub enum DirFile<'a> {
    Doc(Text<'a>), // WIP: Create interface to get content str from FileContent
    Exec(Code<'a>), // WIP: content should be a pointer
}

impl<'a> DirFile<'a> {
    pub fn write_file(&self, current_dir: PathBuf) -> std::io::Result<File> {
        match self {
            DirFile::Doc(text_file) => text_file.create_file(current_dir),
            DirFile::Exec(code_file) => code_file.create_file_and_set_permissions(current_dir),
        }
    }
}

#[derive(Debug)]
pub struct Text<'a> {
    pub file_name: &'a str,
    pub content: Cow<'a, str>
}

#[derive(Debug)]
pub struct Code<'a> {
    pub file: Text<'a>
}

impl<'a> Code<'a> {
    fn create_file_and_set_permissions(&self, current_dir: PathBuf) -> std::io::Result<File> {
        self.file.create_file(current_dir).and_then(|written_file| {
            written_file.metadata().map(|file_metata| {
                use std::os::unix::fs::PermissionsExt as _;

                let mut perms = file_metata.permissions();

                perms.set_mode(0o755);

                let v = written_file.set_permissions(perms); // FIX: fix this

                written_file
            })
        })
    }
}

impl<'a> Text<'a> {
    fn create_file(&self, current_dir: PathBuf) -> std::io::Result<File> {
        let Text{ref file_name, ref content} = self;

        File::create({
            let mut file_dir = current_dir.clone();
            file_dir.push(file_name);
            file_dir
        })
        .and_then(|mut f| f.write_all(content.as_bytes()).map(|()| f))
    }
}
