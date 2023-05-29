use serde::Deserialize;
use config::ConfigError;

#[derive(Deserialize)]
pub struct Config{
    pub server:ServerConfig
}

#[derive(Deserialize)]
pub struct ServerConfig{
    pub host:String,
    pub port:i32
}

impl Config{
    pub fn from_env() -> Result<Self, ConfigError>{
        let mut cfg = config::Config::new(); // creating a new instance of Config Struct from config library
        cfg.merge(config::Environment::new())?; // merging environment variaables
        cfg.try_into() // converting config into our config sstruct using try_into()
    }
}