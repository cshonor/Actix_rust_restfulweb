use tracing::Subscriber;
use tracing_subscriber::{
    EnvFilter,
    Registry,
    fmt::MakeWriter,
    layer::SubscriberExt, // 必须导入：为 Registry 提供 .with() 方法
    util::SubscriberInitExt, // 必须导入：为 Subscriber 提供 .init() 方法
};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};

/// 创建支持 JSON 格式化、环境变量过滤的日志订阅器
/// 
/// # 参数
/// - `name`: 日志的“服务标识”（会作为 JSON 日志的 `service` 字段）
/// - `env_filter`: 外部指定的日志过滤规则（优先级最高，覆盖默认逻辑）
/// - `sink`: 日志输出目标（如 stdout、文件，需实现 MakeWriter）
/// 
/// # 返回
/// 满足全局使用的日志订阅器（支持 Send + Sync + 'static）
pub fn get_subscriber(
    name: String,
    env_filter: EnvFilter,
    sink: impl for<'a> MakeWriter<'a> + Send + Sync + 'static, // 简化泛型约束，与原 where 等价
) -> impl Subscriber + Send + Sync + 'static { // 显式添加 'static，支持全局初始化
    // 修复问题1：优先使用外部传参的 env_filter，而非重新创建
    // （若需保留“传参为空时用默认”，可将参数改为 Option<EnvFilter>，见下方扩展）
    let env_filter = env_filter;

    // 修复问题2：为 JsonStorageLayer 指定默认 Fields 类型（VisitFmt 是 tracing 内置实现）
    let json_storage_layer = JsonStorageLayer::<tracing::field::VisitFmt>::default();

    // 创建 Bunyan JSON 格式化层（将日志转为标准 JSON 格式）
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    // 组合各层：Registry（订阅器注册表）+ 过滤层 + 存储层 + 格式化层
    Registry::default()
        .with(env_filter)           // 1. 日志过滤（按级别/模块筛选，如 "my_app=debug"）
        .with(json_storage_layer)   // 2. 存储 JSON 日志所需的元数据
        .with(formatting_layer)     // 3. 将日志格式化为 JSON 并输出到 sink
}

/// 初始化全局日志订阅器（基于 get_subscriber 创建的实例）
/// 
/// # 注意
/// 程序中应只调用一次，重复初始化会 panic
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync + 'static) {
    // 修复潜在问题：使用 SubscriberInitExt::init()（需导入该 trait）
    // 原代码的 subscriber.init() 依赖该 trait，不导入会编译失败
    subscriber.init();
}