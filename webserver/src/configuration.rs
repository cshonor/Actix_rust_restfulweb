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