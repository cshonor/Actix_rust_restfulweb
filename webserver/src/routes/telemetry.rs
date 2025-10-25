use tracing::Subscriber;
use tracing_subscriber::{EnvFilter, Registry};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};

pub fn get_subscriber(name: String, env_filter: EnvFilter) -> impl Subscriber + Send + Sync {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);
    Registry::default().with(env_filter).with(JsonStorageLayer).with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    subscriber.init();
}