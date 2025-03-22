

pub struct NewDockerProject<'a> {
    pub project_name: &'a str,
    // curr_dir: String,
    pub docker_base_name: &'a str,
    pub docker_options: DockerOptions,
}

pub struct DockerOptions {
    pub x11_support: bool,
    pub nvidia_runtime: bool,
    pub is_debian_based: bool,
}
