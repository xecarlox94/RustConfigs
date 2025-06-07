
pub struct NewDockerProject {
    pub project_name: String,
    // TODO: add current execution directory
    // pub curr_dir: &'a str,
    pub docker_base_name: String,
    pub docker_options: DockerOptions,
    pub dockerfile_content: String,
    pub docker_run_content: String,
}


pub struct DockerOptions {
    pub project_name: String,
    pub docker_base_name: String,
    pub x11_support: bool,
    pub nvidia_runtime: bool,
    pub is_debian_based: bool,
}
