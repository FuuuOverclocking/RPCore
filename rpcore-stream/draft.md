# rpcore-stream

## 层次

- 字节可读可写可通知
  - `ReadHalf`, `WriteHalf`: `extended_io::{Read, Write}`
- 缓冲
  - `RecvBuffer`, `SendBuffer`: `codec::{Decoder, Encoder}`, `recv(b.area(Some([body] + Header::LEN)))`
- 编解码算法
  - `{User Defined Type}`: `Encode`, `Decode`
- 卸载发送器
  - `OffloadSender`: `Callback`
- 主类型
  - `SockStream`: `TryRecvInvocation`

## 逻辑顺序

