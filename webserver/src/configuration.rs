use config::Environment as ConfigEnvironment;
use secrecy::{Secret, ExposeSecret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgSslMode, PgConnectOptions};
use sqlx::ConnectOptions;
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: Secret<String>,
    pub database_name: Secret<String>,
    pub require_ssl: bool
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(Debug, serde::Deserialize)]
pub enum AppEnvironment {
    Local,
    Production,
}

impl AppEnvironment {
    pub fn as_str(&self) -> &'static str {
        match self {
            AppEnvironment::Local => "local",
            AppEnvironment::Production => "production",
        }
    }
}

impl TryFrom<String> for AppEnvironment {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(AppEnvironment::Local),
            "production" => Ok(AppEnvironment::Production),
            other => Err(format!("{} is not a valid environment", other)),
        }
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let environment: AppEnvironment = std::env::var("APP_ENVIRONMENT")
    .unwrap_or_else(|_| "local".into())
    .try_into()
    .expect("Failed to parse APP_ENVIRONMENT");

    let environment_filename = format!("{}.yaml", environment.as_str());
    let settings = config::Config::builder()
    .add_source(config::File::from(configuration_directory.join(&environment_filename)))
    .add_source(config::File::from(configuration_directory.join("base.yaml")))
    .add_source(ConfigEnvironment::with_prefix("APP").prefix_separator("_").separator("__"))
    .build()?
    .try_deserialize::<Settings>()?;
    Ok(settings)
}


impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!("postgres://{}:{}@{}:{}/{}", self.username, self.password.expose_secret(), self.host.expose_secret(), self.port, self.database_name.expose_secret()))
    }
    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!("postgres://{}:{}@{}:{}", self.username, self.password.expose_secret(), self.host.expose_secret(), self.port))
    }
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host.expose_secret())
            .port(self.port)
            .username(&self.username)
            .password(&self.password.expose_secret())
            .ssl_mode(ssl_mode)
    }   


    pub fn with_db(&self) -> PgConnectOptions {
        let mut options = self.without_db()
            .database(&self.database_name.expose_secret());
        options.log_statements(tracing::log::LevelFilter::Trace);
        options
    }
}   