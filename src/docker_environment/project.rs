
pub struct NewDockerProject<'d, 'prj_name, 'base_name> {
    pub project_name: &'prj_name str,
    // TODO: add current execution directory
    // pub curr_dir: &'a str,
    pub docker_base_name: &'base_name str,
    pub docker_options: &'d DockerOptions<'prj_name, 'base_name>,
    pub dockerfile_content: String,
    pub docker_run_content: String,
}


pub struct DockerOptions<'prj_name, 'base_name> {
    pub project_name: &'prj_name str,
    pub docker_base_name: &'base_name str,
    pub x11_support: bool,
    pub nvidia_runtime: bool,
    pub is_debian_based: bool,
}
