use std::{
    fs::{
        self,
        File,
    }, io::{
        self,
        Write,
    }, path::PathBuf
};

impl NewDockerProject {

    pub fn new(
        project_name: String,
        docker_base_name: String,
        x11_support: bool,
        nvidia_runtime: bool,
        is_debian_based: bool,
    )
    -> Self
    {

        NewDockerProject {
            name: project_name,
            docker_base_name,
            docker_options:
                DockerOptions {
                    x11_support,
                    nvidia_runtime,
                    is_debian_based,
                }
        }
    }

    fn get_docker_build_and_runfile(&self) -> String {
        format!(r#"
clear &&
    docker_build.sh &&
    docker_run.sh
        "
            bash
        "
        "
            -v '${{PWD}}/src':/src
            --rm
            --privileged
            --name $PROJECT_FOLDER
        "
        $ADD_OPTS
        "#)
    }

    fn get_dockerfile(&self) -> String {
        format!(
            "FROM {}\n{}\nWORKDIR /src\n\n",
            self.docker_base_name,
            if self.docker_options.is_debian_based
            {
                "ARG DEBIAN_FRONTEND=noninteractive"
            }
            else { "" }
        )
    }

    pub fn bootstrap_docker_project(self, curr_dir: PathBuf) -> Result<(), io::Error> {


        let _ = ProjectDirectory(
            curr_dir,
            Directory(
                "another_dir".to_string(),
                Some(Box::new(vec![
                    PrjFile::DirFile(CodeFile("run.sh".to_string(), "top run fileeeee".to_string())),
                    PrjFile::DirFile(CodeFile("Dockerfile".to_string(), "dockerfileeeee".to_string())),
                    PrjFile::Dir(Directory(
                        "shell_utils".to_string(),
                        Some(Box::new(vec![
                            PrjFile::DirFile(CodeFile("build_docker.sh".to_string(), "docker build util".to_string())),
                            PrjFile::DirFile(CodeFile("run_docker.sh".to_string(), "docker run util".to_string())),
                        ]))
                    )),
                    PrjFile::Dir(Directory(
                        "src".to_string(),
                        Some(Box::new(vec![
                            PrjFile::DirFile(CodeFile("hello.sh".to_string(), "echo \"Hello World\"".to_string())),
                        ]))
                    ))
                ]))
            )
        )
            .build()?;

        Ok(())
    }

}




enum PrjFile {
    Dir(Directory),
    DirFile(CodeFile),
}

impl PrjFile {

    fn create_file_blob(self, current_dir: PathBuf) -> () {
        match self {
            PrjFile::Dir(directory) => directory.create_directory(current_dir),
            PrjFile::DirFile(code_file) => code_file.create_file(current_dir),
        }
    }
}


type Content = String;
type NameBlob = String;

struct CodeFile(NameBlob, Content);


impl CodeFile {

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


struct Directory(
    NameBlob,
    Option<
        Box<
            Vec<PrjFile>
        >
    >
);

impl Directory {


    fn create_directory(self, curr_folder: PathBuf) -> ()
    {
        let Directory(dir_name, maybe_box_dir_contents) = self;

        let mut new_dir = curr_folder.clone();
        new_dir.push(dir_name);

        let _ = fs::create_dir(&new_dir);

        match maybe_box_dir_contents {
            Some(box_dir_contents) => {

                let prjFiles : Vec<PrjFile> = *box_dir_contents;

                let _ = prjFiles
                    .into_iter()
                    .map(
                        |prf_file: PrjFile| {

                            println!("creating file blobs");
                            prf_file.create_file_blob(new_dir.clone());
                        }
                    )
                    .collect::<Vec<_>>();
            },
            None => (),
        }
    }

    fn get_dirname_str(&self) -> String {

        let Directory(dir_name, _): &Directory = self;

        dir_name.to_string()

    }


}






struct ProjectDirectory(PathBuf, Directory);

impl ProjectDirectory {

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













struct DockerOptions {
    x11_support: bool,
    nvidia_runtime: bool,
    is_debian_based: bool,
}

pub struct NewDockerProject {
    name: String,
    // curr_dir: String,
    docker_base_name: String,
    docker_options: DockerOptions,
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
