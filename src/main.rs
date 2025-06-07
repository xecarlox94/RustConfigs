use std::env;

use rust_configs::DockerOptions;

fn main() -> std::io::Result<()> {
    let docker_options = DockerOptions {
        project_name: "sf".to_string(),
        docker_base_name: "ubuntu".to_string(),
        x11_support: true,
        nvidia_runtime: true,
        is_debian_based: true,
    };

    env::current_dir().and_then(|current_dir| {
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
