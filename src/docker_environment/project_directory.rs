use std::{
    path::PathBuf,
};

pub use crate::docker_environment::file::{CodeFile};


pub struct ProjectDirectory<'a>(pub PathBuf, pub Directory<'a>);

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

