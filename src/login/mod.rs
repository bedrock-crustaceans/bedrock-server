mod handler;

use std::time::Duration;
use bedrockrs::proto::connection::Connection;
use shipyard::World;
use crate::entity::Entity;
use crate::error::LoginError;
use crate::entity::player::Player;
use crate::entity::position::Pos;
use crate::entity::velocity::Vel;
use crate::login::handler::LoginHandler;

pub async fn login(connection: Connection, world: &mut World, login_handler: impl LoginHandler) -> Result<(Entity, Player, Pos, Vel), LoginError> {
    let mut shard = connection.into_shard(Duration::from_millis(20), 128).await;
    
    shard.recv();
    
    
    todo!()
    
}