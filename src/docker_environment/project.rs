use std::rc::Rc;



pub struct NewDockerProject<'d> {
    pub project_name: Rc<String>,
    // TODO: add current execution directory
    // pub curr_dir: &'a str,
    pub docker_base_name: Rc<String>,
    pub docker_options: &'d DockerOptions,
    pub dockerfile_content: String,
    pub docker_run_content: String,
}



pub struct DockerOptions {
    pub project_name: Rc<String>,
    pub docker_base_name: Rc<String>,
    pub x11_support: bool,
    pub nvidia_runtime: bool,
    pub is_debian_based: bool,
}
