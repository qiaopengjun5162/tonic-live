# Rust tonic

Protos

Builder

Generated code

Server

Client

Interceptor

## 实操

```bash
cargo generate --git git@github.com:qiaopengjun5162/rust-template.git
cd tonic-live
cargo add tonic
cargo add prost
cargo add tokio --features full
cargo add tonic-build --build
touch abi.proto
cargo deny check licenses
```

## 参考

- <https://github.com/hyperium/tonic>
- <https://docs.rs/tonic/latest/tonic/>
- <https://docs.rs/tonic-build/latest/tonic_build/>
- <https://docs.rs/tonic-build/latest/tonic_build/struct.Builder.html>
- <https://docs.rs/tokio/latest/tokio/sync/broadcast/index.html>
- <https://docs.rs/tokio-stream/latest/tokio_stream/wrappers/struct.UnboundedReceiverStream.html>
