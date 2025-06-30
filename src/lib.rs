use std::{io::Error, path::PathBuf};

mod docker_environment;

use crate::docker_environment::project_directory::{Directory, PrjFile, ProjectDirectory};

use docker_environment::{
    file::{CodeFile, FilePrj, TextFile},
    project::NewDockerProject,
};

pub use docker_environment::project::DockerOptions;

impl<'d, 'dockerfile, 'prj_name, 'base_name> DockerOptions<'prj_name, 'base_name> {
    pub fn get_new_docker_project(&'d self) -> NewDockerProject<'d, 'prj_name, 'base_name> {
        let DockerOptions {
            docker_base_name,
            project_name,
            ..
        } = self;

        NewDockerProject {
            project_name,
            docker_base_name,
            dockerfile_content: self.get_dockerfile(),
            docker_run_content: self.get_docker_build_and_runfile(),
            docker_options: self,
        }
    }

    fn get_dockerfile(&self) -> String {
        format!(
            "FROM {}\n\n{}\n\n\nWORKDIR /src",
            self.docker_base_name,
            if self.is_debian_based.eq(&true) {
                "ARG DEBIAN_FRONTEND=noninteractive"
            } else {
                ""
            }
        )
    }

    fn get_docker_build_and_runfile(&self) -> String {
        let get_char_or_empty = |b: bool, s: String| -> String {
            if b {
                s
            } else {
                "".to_string()
            }
        };

        let x11_nvidia_str = format!(
            "{}{}",
            get_char_or_empty(self.nvidia_runtime, "-n ".to_string()),
            get_char_or_empty(self.x11_support, "-x".to_string()),
        );
        format!(
            r#"

    source shell_utils/utils.sh

    DOCKER_NAME=$(generate_docker_name)

    clear &&\
        echo "building $DOCKER_NAME" &&\
        build_docker_fn "$DOCKER_NAME" || exit 1

    run_docker_fn \
        \
        "\
            bash \
        "\
        \
        "\
            -v '${{PWD}}/src':/src \
            --rm \
            --privileged \
            --name $PROJECT_FOLDER \
        "\
        \
        "$DOCKER_NAME" \
        \
        {}"#,
            x11_nvidia_str
        )
    }
}

impl<'d, 'prj_name, 'base_name> NewDockerProject<'d, 'prj_name, 'base_name> {
    pub fn bootstrap_docker_project(self, curr_dir: PathBuf) -> Result<(), Error> {
        ProjectDirectory(
            curr_dir,
            Directory(
                String::from("another_dir"),
                Some(Box::new([
                    PrjFile::Dir(Directory(
                        String::from("src"),
                        Some(Box::new([PrjFile::DirFile(FilePrj::Code(CodeFile(
                            TextFile(
                                String::from("hello.sh"),
                                String::from("echo \"Hello World\""),
                            ),
                        )))])),
                    )),
                    PrjFile::DirFile(FilePrj::Code(CodeFile(TextFile(
                        String::from("run.sh"),
                        self.docker_run_content.clone(),
                    )))),
                    PrjFile::DirFile(FilePrj::Code(CodeFile(TextFile(
                        String::from("Dockerfile"),
                        self.dockerfile_content.clone(),
                    )))),
                    PrjFile::Dir(Directory(
                        String::from("shell_utils"),
                        Some(Box::new([
                            PrjFile::DirFile(FilePrj::Code(CodeFile(TextFile(
                                String::from("utils.sh"),
                                String::from(
                                    r#"
source ./shell_utils/get_container_name.sh
source ./shell_utils/build_docker.sh
source ./shell_utils/run_docker.sh
                            "#,
                                ),
                            )))),
                            PrjFile::DirFile(self.get_build_docker_util_file()),
                            PrjFile::DirFile(self.get_run_docker_util_file()),
                            PrjFile::DirFile(FilePrj::Code(CodeFile(TextFile(
                                String::from("get_container_name.sh"),
                                String::from(
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
                            "#,
                                ),
                            )))),
                        ])),
                    )),
                ])),
            ),
        )
        .build()

        // eprintln!("EXECUTE BASH SCRIPT TO GO INSIDE PRJ FOLDER AND RUN run.sh");

        // use std::process::Command,
        //
        // Command::new("bash")
        // .args(["./run.sh"])
        // .output()
        // .map(|_| ())
    }

    fn get_build_docker_util_file(&self) -> FilePrj {
        FilePrj::Code(CodeFile(TextFile(
            String::from("build_docker.sh"),
            String::from(
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

            "#,
            ),
        )))
    }

    fn get_run_docker_util_file(&self) -> FilePrj {
        FilePrj::Code(CodeFile(TextFile(
            String::from("run_docker.sh"),
            String::from(
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
$XHOST \
sudo docker run \
-it \
$DOCKER_ARGS \
$ADD_OPTS \
$DOCKER_NAME \
$RUN_CMD \
    "

    clear
    echo "$CMD"
    echo "$CMD" > .command_executed.sh

    eval "$CMD"

}
                "#,
            ),
        )))
    }
}
