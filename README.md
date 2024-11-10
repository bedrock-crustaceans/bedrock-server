# bedrock-server

This repository is a MCPE Server software which uses [`bedrock-rs`](https://github.com/bedrock-crustaceans/bedrockrs) crate.

Feel free to join our [Discord Server](https://discord.gg/b3GQbT72) to ask questions/discute with contributors/enthusiasts.

# Installation

Use the example to setup your server.

## Cargo.toml

```toml
[package]
name = "server"
version = "0.1.0"
edition = "2021"

[dependencies]
bedrock-server = { git = "https://github.com/bedrock-crustaceans/bedrock-server" }
tokio = "1.41.1"
```

## main.rs

```rust
use std::net::SocketAddr;
use bedrock_server::server::builder::ServerBuilder;

#[tokio::main]
async fn main() {
    let mut server = ServerBuilder::new()
        .name("Server")
        .sub_name("bedrock-rs")
        .listener(SocketAddr::new("127.0.0.1".parse().unwrap(), 19132))
        .build()
        .await;
    
    server.start().await;

    loop {
        
    }
    
    server.stop().await;
}
```
