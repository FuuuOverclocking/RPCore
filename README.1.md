Server {
    Message
    RootHandler
}

- 服务，但听起来容易和 Service API 的某某服务混淆，因此我们换个词，叫它 Handler
- 调用：参数，回复方式
- 层次：发出调用者 InvocationEmitter
    - IO 多路复用器
        - 不搞多路，just loop
        - readiness-based: epoll
        - completion-based: io_uring
        - polling-based: 

- 背压：消费端可能跟不上生产端，内存/硬盘会被打爆的
    - 当然了，我们也可以说，不去探究系统极限，直接对连接来个硬限制
- 用有界 mpsc，或者工作窃取，手段多多任君选择


背压：
- 用可写事件驱动整个系统
https://www.cloudwego.io/zh/docs/volo/faq/#poll_ready%E8%83%8C%E5%8E%8B%E5%93%AA%E5%8E%BB%E4%BA%86


自底向上：
- Stream
- Message
- Handler
    - 分发给多个后端，可以用有界 mpmc 队列，工作窃取，...
    - 通过 roo 组合


在 Rust 中，编写一个非常通用而且高效的 RPC 服务器的抽象。

首先，我们有这些假定：
- 不使用异步和 tokio 库
- 对于每个 RPC 调用，有一次响应，不会是 0 个或多个
- 服务器可能是多线程的

我们的抽象开始于

```rs
pub trait Handler {
    type Arg;
    type Ret;

    fn handle<Cb>(&mut self, invocation: Invocation<Self::Arg, Cb>)
    where
        Cb: Callback<Self::Ret>;
}

pub struct Invocation<Arg, Cb> {
    arg: Arg,
    callback: Cb,
}

pub trait Callback<T>: Send + 'static {
    fn call(self, val: T);
}

impl<T, F> Callback<T> for F
where
    F: FnOnce(T) + Send + 'static,
{
    fn call(self, val: T) {
        self(val)
    }
}
```

在我们的抽象中，将会有 InvocationSource，Handler 和 Invocation 将服务器分层：
InvocationSource 提供 Invocation 给 Handler，Handler 可以在其他线程调用 Invocation 中的 callback 完成对消息的回复。

例如，我们可以用 crossbeam::channel::bounded 和 oneshot 来实现一个基于消息通道的 RPC 服务器：
- 客户端拥有 `channel::Sender<Invocation<Arg, OneshotCallback<Ret>>>`
- 服务器拥有 `channel::Receiver<Invocation<Arg, OneshotCallback<Ret>>>`
其中 OneshotCallback 内部是一个 oneshot::Sender.

消息通道已经把数据封装成了“消息”，我们接下来探索一下 TCP Listener、TCP Stream、UDS Listener、Vsock... 它们有些只是个 Listener，需要 accept 得到 Stream；而 Stream 也需要进一步设定消息帧格式，得到消息。

此外 InvocationSource 还有一个重要的事情，就是关于 IO 多路复用，这里存在三个选择：
- 不多路，just loop and read
- readiness-based: epoll
- completion-based: io_uring
需要注意的是，InvocationSource 这个概念不应包含 IO 多路复用器这个数据结构，相反，它应该被注册到 IO 多路复用器上面（如果存在）。另外消息通道可以通过 eventfd 来实现 IO 多路复用。

让我们一步一步来，首先，考虑一下 InvocationSource 以及我们最主要的几个 trait。

MessageFramer

- 阻塞接口 recv
- 非阻塞接口 try_recv
    - polling: 低延迟场景
- readiness 接口
    - init
    - on_ready
- completion 接口
    - init/submit
    - on_completion
        - 通常比 on_ready 多出了一个再次提交操作的步骤

组合可得 Linux 下不同 RPC 服务器的模式:
- 单路 (可选 polling)
- epoll 多路复用 (可选 polling)
- io_uring
    - SQ 可选 通知/轮询
    - CQ 可选 阻塞/轮询

## 1. 同步阻塞式

```rs
loop {
    let invocation = source.recv();
    root_handler.handle(invocation);
}
```

## 2. Readiness-based IO 多路复用

- 一个非阻塞的 try_recv 接口
- 事件订阅者

### 2.1. 衍生出 polling 模式，优化延迟

非阻塞的 try_recv 接口可以拿来忙轮询，可以将延迟从 10us 量级优化到 500ns 量级。