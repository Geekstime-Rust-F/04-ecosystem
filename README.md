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
记录各种各样的日志, 以及为日志提供各种各样的tag以便于其他日志系统跟它很好地交互
