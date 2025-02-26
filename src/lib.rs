use std::{
    fs::{
        self,
        File,
    }, 
    io::{
        self,
        Write,
    }, 
    path::PathBuf
};

impl NewDockerProject {

    pub fn new(
        project_name: String,
        docker_base_name: String,
        x11_support: bool,
        nvidia_runtime: bool,
        is_debian_based: bool,
    )
    -> NewDockerProject
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


        let prj_dir = ProjectDirectory(
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
        );

        create_project_directory(prj_dir)?;

        Ok(())
    }

}


fn create_project_directory(prj_directory: ProjectDirectory) -> Result<(), io::Error>
{

    eprintln!("change this current dir to an immutable directory, use pointers!!!!");

    let ProjectDirectory(current_path, directory) = prj_directory;


    match fs::exists(&current_path) {
        Ok(true) => fs::remove_dir_all(current_path.clone())?,
        Ok(false) => (),
        Err(err) => return Err(err),
    };

    Ok(
        create_directory(current_path, directory)
    )
}

fn create_directory(curr_folder: PathBuf, dir: Directory) -> () 
{
    let Directory(dir_name, maybe_dir_contents) = dir;

    let mut new_dir = curr_folder.clone();
    new_dir.push(dir_name);

    let _ = fs::create_dir(&new_dir);

    maybe_dir_contents.map_or(
        (),
        |dcontents: Box<Vec<PrjFile>>| -> () {
            let _ =dcontents
                .into_iter()
                .map(
                    |prf_file: PrjFile| {
                        create_file_blob(new_dir.clone(), prf_file)
                    }
                );
        }
    )
}

fn create_file_blob(current_dir: PathBuf, prj_file: PrjFile) -> () {
    match prj_file {
        PrjFile::Dir(directory) => create_directory(current_dir, directory),
        PrjFile::DirFile(code_file) => create_file(current_dir, code_file),
    }
}

fn create_file(current_dir: PathBuf, code_file: CodeFile) -> () {

    let CodeFile(file_name, content) = code_file;

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


struct ProjectDirectory(PathBuf, Directory);

enum PrjFile {
    Dir(Directory),
    DirFile(CodeFile),
}

type Content = String;
struct CodeFile(String, Content);

struct Directory(
    String,
    Option<
        Box<
            Vec<PrjFile>
        >
    >
);














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
