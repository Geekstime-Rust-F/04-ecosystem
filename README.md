# Geektime Rust 语言训练营 第四周 Rust生态系统

## 错误处理
### anyhow
anyhow这个库实现了大部分`Error`转换成`anyhow::Error`, 在main函数以及测试中的返回值中使用`anyhow::Error`, 就不用再自己手动转换了.

### thiserror
为标准库中的`std::error::Error`提供了一个方便的derive宏. 即当使用`#[derive(Error, Debug)]`的时候, 实际上是为下面的数据类型实现了Error trait

### anyhow 和 thiserror的对比
在不关心error类型的时候使用anyhow, 一般用在应用程序中. 在做一些library的时候, 往往要设计特定的error类型一般使用者获取信息的时候使用thiserror.
如果使用anyhow时发现需要用到很多的downcast的时候, 也可以考虑使用thiserror了

## 日志处理
### tracing
记录各种各样的日志(也可以记录event, metrics), 以及为日志提供各种各样的tag, 以便于其他日志系统跟它很好地交互

#### span
tracing可以生成一个个的span, 比如在记录一个新的函数的时候可以开启一个新的span, 在函数结束的时候关闭这个span. 这样就可以很好地记录函数的执行时间, 以及函数的调用关系等等. 如果没有指定span的话, 这些日志会打印到一个叫Global的span中. 我们一般会在重要的函数中指定span.

##### instrument
我们可以在函数上使用
```rust
#[tracing::instrument]
```
这个宏会自动创建一个span, 并且在函数调用时进入这个span, 在函数结束的时候关闭这个span.

##### span!
我们也可以使用
```rust
let span = tracing::span!(tracing::Level::INFO, "hello", name = "world");
let _guard = span.enter();
```
这个宏会创建一个span, 并且在这个span中记录一些信息, 比如name = "world"

### tracing-subscriber
我们需要使用一个collector来收集日志, 比如输出到控制台, 输出到文件, 输出到其他日志系统等等. 这个库提供了一些常用的collector, 比如`fmt`(它提供了一些默认值, 输出到控制台).
初始化一个全局的collector:
```rust
tracing_subscriber::fmt::init();
```

### tracing-appender
提供了一种非阻塞的日志记录方式(通过一个单独的logging thread来记录日志), 这样可以避免在记录日志的时候阻塞主线程.

#### 滚动收集日志
当日志达到一定大小或时间间隔时, 自动创建新的日志文件.
```rust
use tracing::info;
use tracing_subscriber::fmt::Subscriber;
use tracing_appender::rolling::daily;

fn main() {
    // 创建一个每天滚动的日志文件
    let file_appender = daily("/path/to/logs", "myapp.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // 设置日志记录器
    let subscriber = Subscriber::builder()
        .with_writer(non_blocking)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Unable to set global subscriber");

    // 记录一些日志信息
    info!("This is an info message.");
}
```
上述代码创建了一个每天滚动的日志文件, 并且将日志记录到这个文件中. 文件名类似于ecosystem.log.2024-06-30.

### open-telemetry
这个组织提供了很多库, 比如opentelemetry-jaeger, opentelemetry-prometheus等等方便地与其他日志系统交互.
我们先使用opentelemetry-otlp这个库, 直接与otlp打交道, 这样就不用绑定到一个特定的日志系统上了.
```rust
let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic() // use gRPC 作为传输layer
                .with_endpoint("http://localhost:4317"),
        )
        .with_trace_config(
            trace::config()
                .with_id_generator(RandomIdGenerator::default())
                .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    "axum-tracing",
                )])),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;
```
使用tracing-opentelemetry这个库, 可以将tracing的日志记录到opentelemetry中. 使用刚才的tracer生成一个layer, 然后将这个layer加入到tracing的subscriber中.
```rust
let tracer_layer = tracing_opentelemetry::OpenTelemetryLayer::new(tracer).with_filter(LevelFilter::INFO);
```

OpenTelemetry提供两种export数据的方式, 一种时应用程序直接export, 另一种是使用OpenTelemetry Collector来export.

在Collector内部设计中, 一套数据的流入、处理、流出的过程称为pipeline. pipeline由三个部分组成: receiver, processor, exporter. receiver用于接收数据, processor用于处理数据, exporter用于导出数据.

例子中使用`docker run -d -p 16686:16686 -p 4317:4317 -e COLLECTOR_OTLP_ENABLED=true jaegertracing/all-in-one:latest`启动一个jaeger的all-in-one容器, 用作Collector. 其中4317是OTLP的端口, 16686是jaeger的web interface的端口.

### 宏
#### derive_builder
这个宏实现了builder pattern. 通过在struct上使用`#[derive(Builder)]`, 会自动生成一个builder struct, 以及一个build方法和所有fields的setter方法. 这样就可以很方便地构建一个struct了, 对于fields很多或者构建复杂的struct, 这个宏很有用.

#### derive_more
Rust中有许多为基本类型提供的trait, 比如Add, Sub, Mul, From, Display等等. 但对于把这些基本类型包起来的自定义类型就没有实现了, 需要我们手动创建. 当我们自定义类型非常简单的时候, 这就比较annoying. 这个宏就是为了解决这个问题的, 允许我们通过derive的方式实现这些trait
