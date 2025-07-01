use serde::Deserialize;


#[derive(Deserialize)]
pub struct Config {
    pub(crate) secret: String,
}