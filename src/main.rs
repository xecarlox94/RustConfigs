use std::env;
use std::fs;


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

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let path = env::current_dir()?;
    let path_str: String = path.display().to_string();
    println!("current path: {path_str}");

    let new_docker_project: NewDockerProject = get_new_docker_project();

    Ok(())
}

fn get_new_docker_project() -> NewDockerProject {

    NewDockerProject {
        name: String::from("sf"),
        docker_base_name: String::from("ubuntu"),
        docker_options:
            DockerOptions {
                x11_support: true,
                nvidia_runtime: true,
                is_debian_based: true,
            }
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
