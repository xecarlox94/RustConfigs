use std::{
    env, 
    fs,
    io,
    path::PathBuf
};


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut path: PathBuf = env::current_dir()?;


    let project_name: String = "sf".to_string();
    let docker_base_name: String = "ubuntu".to_string();
    let x11_support: bool = true;
    let nvidia_runtime: bool = true;
    let is_debian_based: bool = true;

    let new_docker_project: NewDockerProject 
        = NewDockerProject::new(
            project_name, 
            docker_base_name, 
            x11_support, 
            nvidia_runtime, 
            is_debian_based
        );

    new_docker_project.bootstrap_docker_project(&mut path);

    println!("RAN CODE!");

    Ok(())
}



struct DockerOptions {
    x11_support: bool,
    nvidia_runtime: bool,
    is_debian_based: bool,
}

struct NewDockerProject {
    name: String,
    docker_base_name: String,
    docker_options: DockerOptions,
}

impl NewDockerProject {

    fn new(
        project_name: String, 
        docker_base_name: String, 
        x11_support: bool, 
        nvidia_runtime: bool, 
        is_debian_based: bool
    ) 
    -> NewDockerProject {

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

    fn bootstrap_docker_project(&self, current_dir: &mut PathBuf) -> Result<(), io::Error> {

        current_dir.push("new_dir");

        //let curr_path_string: String = current_dir.display().to_string();

        let curr_dir_str: String = current_dir.display().to_string();

        fs::create_dir(current_dir)?;

        println!("CURRENT_DIR: {curr_dir_str}");

        Ok(())
    }

}



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
