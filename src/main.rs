use std::env;

use rust_configs::NewDockerProject;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let curr_path = env::current_dir()?;

    build_testing_docker_project().bootstrap_docker_project(curr_path)?;

    Ok(())
}


fn build_testing_docker_project() -> NewDockerProject {

    let project_name = "sf".to_string();
    let docker_base_name = "ubuntu".to_string();
    let x11_support = true;
    let nvidia_runtime = true;
    let is_debian_based = true;

    NewDockerProject::new(
        project_name,
        docker_base_name,
        x11_support,
        nvidia_runtime,
        is_debian_based,
    )
}

