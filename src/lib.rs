use std::{
    io::
        Error
    ,
    path::PathBuf,
};


mod docker_environment;


use crate::docker_environment::{
    project_directory::{
        PrjFile,
        Directory,
        ProjectDirectory,
    },
    project::DockerOptions,
};


use docker_environment::file::{CodeFile, FilePrj, TextFile};
pub use docker_environment::project::NewDockerProject;



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

    pub fn bootstrap_docker_project(self, curr_dir: PathBuf) -> Result<(), Error> {

        ProjectDirectory(
            curr_dir,
            Directory(
                "another_dir",
                Some(Box::new([
                    PrjFile::Dir(Directory(
                        "src",
                        Some(Box::new([
                            PrjFile::DirFile(
                                FilePrj::Code(
                                    CodeFile(
                                        TextFile(
                                        "hello.sh",
                                        "echo \"Hello World\""
                                        )
                                    )
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
        .build()?;

        // eprintln!("EXECUTE BASH SCRIPT TO GO INSIDE PRJ FOLDER AND RUN run.sh");


        // use std::process::Command,
        //
        // Command::new("bash")
            // .args(["./run.sh"])
            // .output()
            // .map(|_| ())


        Ok(())

    }


    fn get_dockerfile(&self) -> FilePrj {

        let non_int_debian_str =
            if self.docker_options.is_debian_based
            {
                "ARG DEBIAN_FRONTEND=noninteractive"
            }
            else { "" };

        FilePrj::Code(
        CodeFile(
            "Dockerfile",
            format!(
                "FROM {}\n\n{}\n\n\nWORKDIR /src",
                self.docker_base_name,
                non_int_debian_str
            )
        )
        )
    }



    fn get_docker_utils_dir(&self) -> Directory {
        Directory(
            "shell_utils",
            Some(Box::new([
                PrjFile::DirFile(
                    FilePrj::Code(
                        CodeFile(
                            TextFile(
                            "utils.sh",
                            r#"
source ./shell_utils/get_container_name.sh
source ./shell_utils/build_docker.sh
source ./shell_utils/run_docker.sh
                            "#
                            )
                        )
                    )
                ),
                PrjFile::DirFile(self.get_build_docker_util_file()),
                PrjFile::DirFile(self.get_run_docker_util_file()),
                PrjFile::DirFile(
                    CodeFile(
                        "get_container_name.sh",
                        r#"

generate_docker_name () {
    DOCKER_NAME=$(\
        echo $PWD |\
            cut -c2- |\
            tr '[:upper:]' '[:lower:]' |\
            sed "s/ /_/g;s/-/_/g;s/\//_/g;s/\.//g;"
    )
    DOCKER_NAME="$DOCKER_NAME:latest"
    echo $DOCKER_NAME
}
                        "#.to_owned()
                    )
                ),
            ]))
        )
    }


    fn get_docker_build_and_runfile(&self) -> CodeFile {

        let get_char_or_empty = |b: bool, s: &'a str| -> &'a str {
            if b {
                s
            }
            else {
                ""
            }
        };

        let x11_nvidia_str = format!(
            "{}{}",
            get_char_or_empty(self.docker_options.nvidia_runtime, "-n "),
            get_char_or_empty(self.docker_options.x11_support, "-x"),
        );

        CodeFile(
            "run.sh",
            format!(r#"

source shell_utils/utils.sh

DOCKER_NAME=$(generate_docker_name)

clear &&\
    echo "building $DOCKER_NAME" &&\
    \
    build_docker_fn "$DOCKER_NAME" || exit 1

run_docker_fn \
    \
    "\
        bash \
    "\
    \
    "\
        -v '${{PWD}}/src':/src
        --rm
        --privileged
        --name $PROJECT_FOLDER
    "\
    \
    "$DOCKER_NAME" \
    \
    {}
            "#,
            x11_nvidia_str
            ),
            true
        )
    }


    fn get_build_docker_util_file(&self) -> CodeFile {
        CodeFile(
            "build_docker.sh",
            r#"

build_docker_fn () {

    DOCKER_NAME=$1

    #sudo docker \
        #system prune \
            #-a \
            #--filter "until=4w"

    # sudo docker rmi $(docker images -f dangling=true)
    # sudo docker volume rm $(sudo docker volume ls -q -f dangling=true)


    sudo \
        docker build . \
            -t "$DOCKER_NAME"

            #
            #
            #  I need to add ARG environments to extend further things, such as:
            #
            #  adding user to container user group
            #
            #--build-arg BUILD_ENV=dev

}

            "#.to_string(),
            true
        )
    }


    fn get_run_docker_util_file(&self) -> CodeFile {
        FilePrj::Code(
            CodeFile(
                "run_docker.sh",
                r#"

run_docker_fn () {

    if [ $# -lt 3 ];
    then
        echo "not enough args"
        exit 1
    fi


    RUN_CMD="$1"

    DOCKER_ARGS="$2"

    DOCKER_NAME="$3"

    echo "4: $4"
    echo "5: $5"


    shift 3


    X11=false
    NVIDIA=false

    while getopts 'xn:t:' OPTION;
    do
        case "$OPTION" in
            x)
                X11=true
                ;;
            n)
                NVIDIA=true
                ;;
            *)
                echo "NOTHING"
                ;;
        esac
    done



    # # --- Pulse Audio / Mic and Speakers - Too much to comment, but it's all needed... I think ----------------------
    #     -v /dev/snd:/dev/snd  \
    #     -v /run/user/$uid/puslse:/run/user/$uid/pulse \
    #     -v /dev/shm:/dev/shm \
    #     -v /etc/machine-id:/etc/machine-id \
    #     -v /var/lib/dbus:/var/lib/dbus \
    #     -v ~/.pulse:/home/$dockerUsername/.pulse \
    #     -v ~/.config/pulse/cookie:/root/.config/pulse/cookie \
    #     -e PULSE_SERVER=unix:${XDG_RUNTIME_DIR}/pulse/native \
    #     -v /dev/bus/usb:/dev/bus/usb \
    #     -v ${XDG_RUNTIME_DIR}/pulse/native:${XDG_RUNTIME_DIR}/pulse/native \
    #     --device /dev/snd \
    # # ----------------------------


    # USER PERMISSION
    # https://vsupalov.com/docker-shared-permissions/
    # --user \"$(id -u):$(id -g)\" \


    #-e TERM \
        #-e QT_X11_NO_MITSHM=1 \
        #-e XAUTHORITY=/tmp/.dockerw_b717qf.xauth \
        #-v /tmp/.dockerw_b717qf.xauth:/tmp/.dockerw_b717qf.xauth \
        #-v /tmp/.X11-unix:/tmp/.X11-unix \




    X11_OPTS="\
    -e TERM \
    -e DISPLAY=unix$DISPLAY \
    -v /tmp/.X11-unix:/tmp/.X11-unix:rw \
    "

    NVIDIA_OPTS="\
    -e NVIDIA_VISIBLE_DEVICES=${NVIDIA_VISIBLE_DEVICES:-all} \
    -e NVIDIA_DRIVER_CAPABILITIES=${NVIDIA_DRIVER_CAPABILITIES:-all} \
    --runtime=nvidia \
    --gpus all \
    "


    XHOST=""
    ADD_OPTS=""


    if $X11;
    then
        XHOST="xhost +local:root && "
        ADD_OPTS="$X11_OPTS "
    fi


    if $NVIDIA;
    then
        ADD_OPTS="$ADD_OPTS $NVIDIA_OPTS"
    fi

    CMD="\
sudo docker run \
-it \
$DOCKER_NAME \
    "

    #CMD="\
#$XHOST \
#sudo docker run \
#-it \
#$DOCKER_ARGS \
#$ADD_OPTS \
#$DOCKER_NAME \
#$RUN_CMD \
    #"

    echo "$CMD" > .command_executed.sh

    eval "$CMD"

}
                "#.to_string(),
            false
            )
        )
    }


}
