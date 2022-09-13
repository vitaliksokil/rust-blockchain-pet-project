use crate::*;

#[derive(Debug, PartialEq)]
pub enum ConfigEnv {
    Dev,
    Live
}


pub const ENV: ConfigEnv = ConfigEnv::Dev;