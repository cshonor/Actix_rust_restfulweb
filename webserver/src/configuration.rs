use config::{Config, ConfigError, File, Environment};
use serde::Deserialize;


#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let  settings = config::Config::builder()
   .add_source(config::File::new("configuration.yaml", config::FileFormat::Yaml))
   .build()?
   .try_deserialize::<Settings>()?;
   Ok(settings)
}


impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}", self.username, self.password, self.host, self.port, self.database_name)
    }
}   