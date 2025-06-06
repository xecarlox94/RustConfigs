

pub struct NewDockerProject<'a, 'b> {
    pub project_name: &'a str,
    // TODO: add current execution directory
    // pub curr_dir: &'a str,
    pub docker_base_name: &'b str,
    pub docker_options: DockerOptions<'b>,
    pub dockerfile_content: String,
    pub docker_run_content: String,
}


pub struct DockerOptions<'a> {
    pub docker_base_name: &'a str,
    pub x11_support: bool,
    pub nvidia_runtime: bool,
    pub is_debian_based: bool,
}
