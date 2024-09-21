use std::time::Duration;
use bedrockrs::proto::connection::Connection;
use crate::entity::entity::Entity;
use crate::error::LoginError;
use crate::entity::player::Player;

pub async fn login(connection: Connection) -> Result<(Entity, Player), LoginError> {
    let mut shard = connection.into_shard(Duration::from_millis(20), 128).await;
    
    shard.recv();
    
    
    todo!()
    
}