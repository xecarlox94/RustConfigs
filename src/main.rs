use std::env;

use rust_configs::NewDockerProject;


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let curr_path = env::current_dir()?;

    let project_name = "sf";
    let docker_base_name = "ubuntu";
    let x11_support = true;
    let nvidia_runtime = true;
    let is_debian_based = true;

    let docker_prj = NewDockerProject::new(
        project_name,
        docker_base_name,
        x11_support,
        nvidia_runtime,
        is_debian_based,
    );

    docker_prj.bootstrap_docker_project(curr_path)?;

    // eprintln!("ADD RC<STR> TO PROGRAM, INSTEAD OF COPY STRING (FOR IMMUTABLE CASES)");
    // eprintln!("ADD RC<[T]> TO PROGRAM, INSTEAD OF VECTOR COPYING (FOR IMMUTABLE CASES)");
    // eprintln!("wrapper for creating bash files with shebangs");
    // eprintln!("functon to create executable bash files");
    // eprintln!("Run the initial run.sh command");

    Ok(())
}
