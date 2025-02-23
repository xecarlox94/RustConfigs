use std::env;
use std::path::PathBuf;

use rust_configs::{
    NewDockerProject,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut curr_path: PathBuf = env::current_dir()?;


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

    println!("\n\nDOCKER FILE\n\n");
    println!("{}", new_docker_project.get_dockerfile());

    println!("\n\nDOCKER RUN\n\n");
    println!("{}", new_docker_project.get_docker_buildrun_file());

    println!("\n\nDOCKER bootstrap\n\n");
    new_docker_project.bootstrap_docker_project(&mut curr_path)?;

    Ok(())
}
