use std::net::SocketAddr;
use bedrock_server::server::builder::ServerBuilder;

#[tokio::main]
async fn main() {
    let mut server = ServerBuilder::new()
        .name("Server")
        .sub_name("bedrock-rs")
        .listener(SocketAddr::new("127.0.0.1".parse().unwrap(), 19132))
        .build().await;
    
    server.start().await;

    loop {
        
    }
    
    server.stop().await;
}
