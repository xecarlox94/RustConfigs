use std::{
    fs::{
        self,
        File,
    }, io::{
        self,
        Write,
    }, path::PathBuf
};



pub struct NewDockerProject<'a> {
    project_name: &'a str,
    // curr_dir: String,
    docker_base_name: &'a str,
    docker_options: DockerOptions,
}

struct DockerOptions {
    x11_support: bool,
    nvidia_runtime: bool,
    is_debian_based: bool,
}



impl<'a> NewDockerProject<'a> {

    pub fn new(
        project_name: &'a str,
        docker_base_name: &'a str,
        x11_support: bool,
        nvidia_runtime: bool,
        is_debian_based: bool,
    )
    -> Self
    {

        NewDockerProject {
            project_name,
            docker_base_name,
            docker_options:
                DockerOptions {
                    x11_support,
                    nvidia_runtime,
                    is_debian_based,
                }
        }
    }

    pub fn bootstrap_docker_project(self, curr_dir: PathBuf) -> Result<(), io::Error> {

        ProjectDirectory(
            curr_dir,
            Directory(
                "another_dir",
                Some(Box::new([
                    PrjFile::Dir(Directory(
                        "src",
                        Some(Box::new([
                            PrjFile::DirFile(
                                CodeFile(
                                    "hello.sh",
                                    "echo \"Hello World\"".to_string()
                                )
                            ),
                        ]))
                    )),
                    PrjFile::DirFile(self.get_docker_build_and_runfile()),
                    PrjFile::DirFile(self.get_dockerfile()),
                    PrjFile::Dir(self.get_docker_utils_dir()),
                ]))
            )
        )
        .build()

    }


    fn get_dockerfile(&self) -> CodeFile {

        let non_int_debian_str =
            if self.docker_options.is_debian_based
            {
                "ARG DEBIAN_FRONTEND=noninteractive"
            }
            else { "" };

        CodeFile(
            "Dockerfile",
            format!(
                "FROM {}\n\n{}\n\n\nWORKDIR /src",
                self.docker_base_name,
                non_int_debian_str
            )
        )
    }


    fn get_docker_build_and_runfile(&self) -> CodeFile {

        eprintln!("create executable files!!!!");

        CodeFile(
            "run.sh",
            format!(r#"

source shell_utils/build_docker.sh
source shell_utils/run_docker.sh

hello_world_build_docker

hello_world_run_docker


\\ clear &&
    \\ docker_build.sh &&
    \\ docker_run.sh
        \\ "
            \\ bash
        \\ "
        \\ "
            \\ -v '${{PWD}}/src':/src
            \\ --rm
            \\ --privileged
            \\ --name $PROJECT_FOLDER
        \\ "
        \\ $ADD_OPTS
            "#)
        )
    }

    fn get_build_docker_util_file(&self) -> CodeFile {
        CodeFile(
            "build_docker.sh",
            r#"
hello_world_build_docker () {
   echo 'hello, world! from build docker!'
}
            "#.to_string()
        )
    }

    fn get_run_docker_util_file(&self) -> CodeFile {
        CodeFile(
            "run_docker.sh",
            r#"
hello_world_run_docker () {
   echo 'hello, world! from run docker!'
}
            "#.to_string()
        )
    }

    fn get_docker_utils_dir(&self) -> Directory {
        Directory(
            "shell_utils",
            Some(Box::new([
                PrjFile::DirFile(self.get_build_docker_util_file()),
                PrjFile::DirFile(self.get_run_docker_util_file()),
            ]))
        )
    }

}




enum PrjFile<'a> {
    Dir(Directory<'a>),
    DirFile(CodeFile<'a>),
}

impl<'a> PrjFile<'a> {

    fn create_file_blob(self, current_dir: PathBuf) -> () {
        match self {

            PrjFile::Dir(directory) => directory.create_directory(current_dir),
            PrjFile::DirFile(code_file) => code_file.create_file(current_dir),
        }
    }
}


type Content = String;

struct CodeFile<'a>(&'a str, Content);


impl<'a> CodeFile<'a> {

    fn create_file(self, current_dir: PathBuf) -> () {

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


struct Directory<'a>(
    &'a str,
    Option<
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

        let _ = fs::create_dir(&new_dir);

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






struct ProjectDirectory<'a>(PathBuf, Directory<'a>);

impl<'a> ProjectDirectory<'a> {

    fn build(self) -> Result<(), io::Error>
    {

        eprintln!("change this current dir to an immutable directory, use pointers!!!!");

        let ProjectDirectory(current_path, directory) = self;

        let mut dir_to_be_created = current_path.clone();

        dir_to_be_created.push(directory.get_dirname_str());


        match fs::exists(&dir_to_be_created) {

            Err(err) => return Err(err),

            Ok(true) => fs::remove_dir_all(dir_to_be_created.clone())?,

            Ok(false) => (),
        };

        Ok(
            directory.create_directory(current_path)
        )
    }

}




    /*

    echo "sudo to create run.sh executable file"
    touch_x run.sh


    mkdir src


    echo -e "clear &&\\
        docker_build.sh &&\\
        docker_run.sh \\
            \"\\
                bash \\
            \"\\
            \"\\
                -v '\${PWD}/src':/src \\
                --rm \\
                --privileged \\
                --name $PROJECT_FOLDER \\
            \"\\
            $ADD_OPTS" > run.sh


    ./run.sh
    */


// Requirements docker bootstrap
//
// selects project name
//
// selects docker image
//
// asks if it is debian
//
// asks if X11 support is needed
//
// asks if nvidia runtime is needed
//
// generates initial dockerfile
//
// generates an executable file called run.sh
//
//      it generates a cmd string
//      it generates a docker args with sane defaults
//
// example:
//
// echo -e "clear &&\\
//     docker_build.sh &&\\
//     docker_run.sh \\
//         \"\\
//             bash \\
//         \"\\
//         \"\\
//             -v '\${PWD}/src':/src \\
//             --rm \\
//             --privileged \\
//             --name $PROJECT_FOLDER \\
//         \"\\
//         $ADD_OPTS" > run.sh


// Requirements docker build
//
// removes old dangling images and volumes
//
// gen_container_name from current directory
//
// runs docker run


// Requirements docker run
//
// checks run arguments for Nvidia and X11
//
//
//     CMD="\
//     $XHOST \
//     sudo docker run \
//     -it \
//     -v $HOME/.config/.FILES:/root/.config/.FILES \
//     $DOCKER_ARGS \
//     $ADD_OPTS \
//     $DOCKER_NAME \
//     $RUN_CMD \
//     "
