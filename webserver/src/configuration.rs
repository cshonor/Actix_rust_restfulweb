#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::builder();
    settings.add_source(config::File::new("configuration.yaml", config::FileFormat::Yaml));
   .build()?
   .try_deserialize::<Settings>()
}
