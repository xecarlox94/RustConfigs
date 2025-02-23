use std::{
    env,
    path::PathBuf,
};


fn main() -> Result<(), Box<dyn std::error::Error>>{

    let mut curr_path: PathBuf = env::current_dir()?;

    let new_docker_project =
        rust_configs::
            NewDockerProject::new(
                "sf".to_string(),
                "ubuntu".to_string(),
                true,
                true,
                true,
            );

    println!(
        "\n\nDOCKER FILE\n\n{}",
        new_docker_project.get_dockerfile()
    );

    println!(
        "\n\nDOCKER RUN\n\n{}",
        new_docker_project.get_docker_buildrun_file()
    );

    println!(
        "\n\nDOCKER bootstrap\n\n"
    );

    new_docker_project.bootstrap_docker_project(&mut curr_path)?;

    Ok(())
}


