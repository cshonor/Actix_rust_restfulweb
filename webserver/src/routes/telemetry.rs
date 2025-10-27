use tracing::Subscriber;
use tracing_subscriber::{
    EnvFilter,
    Registry,
    fmt::MakeWriter,
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};

/// 创建支持 JSON 格式化、环境变量过滤的日志订阅器
/// 
/// # 参数
/// - `name`: 日志的“服务标识”
/// - `env_filter`: 外部指定的过滤规则字符串（如 "my_app=debug"）
/// - `sink`: 日志输出目标（如 stdout、文件）
/// 
/// # 优先级
/// 外部传参 `env_filter` > 环境变量 `RUST_LOG` > 默认值 "info"
pub fn get_subscriber(
    name: String,
    env_filter: String,
    sink: impl for<'a> MakeWriter<'a> + Send + Sync + 'static,
) -> impl Subscriber + Send + Sync + 'static {
    // 正确逻辑：优先用外部传参，若传参无效（如空字符串），再尝试环境变量，最后用默认值
    let env_filter = EnvFilter::new(&env_filter) // 先尝试解析外部传参
        .or_else(|_| EnvFilter::try_from_default_env()) // 传参无效时，尝试环境变量
        .unwrap_or_else(|_| EnvFilter::new("info")); // 最终 fallback 到默认值

    let json_storage_layer = JsonStorageLayer::<tracing::field::VisitFmt>::default();
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(env_filter)
        .with(json_storage_layer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync + 'static) {
    subscriber.init();
}