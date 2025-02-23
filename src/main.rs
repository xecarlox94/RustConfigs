use std::env;


fn main() -> Result<(), Box<dyn std::error::Error>>{

    let curr_path = env::current_dir()?;

    let new_docker_project =
        rust_configs::
            NewDockerProject::new(
                "sf".to_string(),
                "ubuntu".to_string(),
                true,
                true,
                true,
            );

    new_docker_project.bootstrap_docker_project(curr_path)?;

    Ok(())
}


