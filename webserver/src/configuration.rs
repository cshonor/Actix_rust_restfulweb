use config::{Config, ConfigError, File, Environment};
use serde::Deserialize;
use secrecy::{Secret, ExposeSecret};
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: Secret<String>,
    pub database_name: Secret<String>,
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
        format!("postgres://{}:{}@{}:{}/{}", self.username, self.password.expose_secret(), self.host.expose_secret(), self.port, self.database_name.expose_secret())
    }
    pub fn connection_string_without_db(&self) -> String {
        format!("postgres://{}:{}@{}:{}", self.username, self.password.expose_secret(), self.host.expose_secret(), self.port)
    }
}   