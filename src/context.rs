use crate::get_config::Config;

#[derive(Debug)]
pub struct Context {
    pub is_ci: bool,
    pub is_pr: bool,
    pub branch: String,
    pub config: Config,
}
