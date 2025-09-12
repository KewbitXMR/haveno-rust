use serde::Deserialize;


#[derive(Deserialize)]
pub struct Config {
    pub(crate) secret: String,
    pub(crate) node_host: String,
    pub(crate) node_port: i32,
    pub(crate) version: String,
    pub(crate) p2p_version: String
}