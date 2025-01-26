# 如何在 Rust 构建一个好的 RPC Server 抽象

假设:
- 不引入 async runtime
- 对每个 RPC call, 恰好有一个响应, 不会是 0 个或多个

宏观结构:
- Layer
- 承上启下 Handler
- Invocation
- InvocationSource
    - 单路 (可选 polling)
        - recv
        - (polling) try_recv
    - epoll 多路 (可选 polling)
        - try_recv
        - subscribe
        - on_ready
    - io_uring 多路 (SQ 和 CQ 分别可选 polling)
        - submit
        - on_complete

常见的 InvocationSource:
- socket
    - 需要做两件事
        - listener -> Acceptor -> stream
        - byte stream -> Codec -> message
- mpsc
