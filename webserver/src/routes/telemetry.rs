use tracing::Subscriber;
use tracing_subscriber::{EnvFilter, Registry,fmt::MakeWriter};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};

pub fn get_subscriber(name: String, env_filter: EnvFilter, sink:Sink) -> impl Subscriber + Send + Sync 
      where Sink: for<'a> MakeWriter<'a>+Send+Sync+ 'static
{
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    Registry::default().with(env_filter).with(JsonStorageLayer).with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    subscriber.init();
}