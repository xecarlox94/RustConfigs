use std::env;

use clap::{arg, command, Arg, ArgAction, Command};

use rust_configs::DockerOptions;

fn main() -> std::io::Result<()> {
    let matches = command!()
        .propagate_version(true)
        .before_help("Welcome!!")
        .about("Simple utility to bootstrap custom docker environment. I hate Docker so I created a simple program to make it usable. Using Rust is a overkill to template bash scripts but I wanted to learn it. It was fun :)")
        .after_help("Enjoy it!")
        .author("Jose Fernandes, jf94.uk@gmail.com")
        .arg_required_else_help(true)
        .arg(arg!(-p --project <PROJECT> "Sets project name").required(true))
        .arg(arg!(-b --base_image <BASE_IMAGE> "Sets default docker base image").required(true))
        .arg(
            Arg::new("x11_support")
                .short('x')
                .long("x11")
                .help("adds support for x11 desktop application integration")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("nvidia_support")
                .short('n')
                .long("nvidia")
                .help("adds support to nvidia applications")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("debian_support")
                .short('d')
                .long("debian")
                .help("adds support for debian based docker images")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    match (
        matches.get_one::<String>("project"),
        matches.get_one::<String>("base_image"),
    ) {
        (Some(project), Some(base_image)) => {
            let docker_options = DockerOptions {
                project_name: project,
                docker_base_name: base_image,
                x11_support: matches.get_flag("x11_support"),
                nvidia_runtime: matches.get_flag("nvidia_support"),
                is_debian_based: matches.get_flag("debian_support"),
            };

            env::current_dir().and_then(|current_dir| {
                eprintln!("FIX file executable issue");

                docker_options
                    .get_new_docker_project()
                    .bootstrap_docker_project(current_dir)
            })

            // eprintln!("ADD RC<STR> TO PROGRAM, INSTEAD OF COPY STRING (FOR IMMUTABLE CASES)");
            // eprintln!("ADD RC<[T]> TO PROGRAM, INSTEAD OF VECTOR COPYING (FOR IMMUTABLE CASES)");
            // eprintln!("wrapper for creating bash files with shebangs");
            // eprintln!("functon to create executable bash files");
            // eprintln!("Run the initial run.sh command");
        }
        _ => panic!("ERROR"),
    }
}
