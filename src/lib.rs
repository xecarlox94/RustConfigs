use std::{
    env::current_dir, fs::{
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


        eprintln!("change this current dir to an immutable directory, use pointers!!!!");
        let mut current_dir = curr_dir.clone();
        current_dir.push("new_dir");

        match fs::exists(&current_dir) {
            Ok(true) => fs::remove_dir_all(current_dir.clone())?,
            Ok(false) => (),
            Err(err) => return Err(err),
        };

        let curr_dir: String = current_dir.display().to_string();

        let mut src_dir = current_dir.clone();
        src_dir.push("src");

        //let curr_path_string: String = current_dir.display().to_string();

        let curr_dir_str: String = current_dir.display().to_string();

        fs::create_dir(&curr_dir)?;
        fs::create_dir(&src_dir)?;


        let docker_file_path = {
            let mut file_dir = current_dir.clone();
            file_dir.push("Dockerfile");
            file_dir
        };

        let mut docker_file = File::create(docker_file_path)?;

        docker_file.write_all(
            self.get_dockerfile().as_bytes()
        )?;


        println!(
            "\n\nDOCKER RUN\n\n{}",
            self.get_docker_build_and_runfile()
        );

        // fn create_exec_and_run(&self) -> String {
            // touch_x run file
                // paste content run file
                // run file
        // }
        //}

        println!("CURRENT_DIR: {curr_dir_str}");

        let _ = ProjectDirectory(
            curr_dir.clone(),
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

        Ok(())
    }

}


fn createProjectDirectory(prjDirectory: ProjectDirectory) -> () 
{
    let ProjectDirectory(current_path, directory) = prjDirectory;

    createDirectory(directory, current_path)
}

fn createDirectory(dir: Directory, curr_folder: PathBuf) -> () 
{
    let Directory(dir_name, maybe_dir_contents) = dir;

    let mut new_dir = curr_dir.clone();
    new_dir.push(dir_name);

    maybe_dir_contents.map_or(
        (),
        |dcontents: Vec<PrjFile>| -> () {
            dcontents
                .into_iter()
                .map(
                    |prfFile: PfjFile| {
                        createFileBlob(new_dir.clone(), prfFile)
                    }
                )
        }
    )
}

fn createFileBlob(current_dir: PathBuf, pfjFile: PfjFile) -> () {
    match pfjFile {
        Dir(directory) => current_dir(current_dir, directory),
        DirFile(codeFile) => createFile(current_dir, codeFile),
    }
}

fn createFile(current_dir: PathBuf, codeFile: CodeFile) -> () {
    // todo!("yet to finish this function")
    ()
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
