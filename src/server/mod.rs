use std::error::Error;
use bedrockrs::proto::listener::Listener;
use shipyard::World;
use crate::entity::Entity;
use crate::entity::player::Player;
use crate::entity::position::Pos;
use crate::entity::velocity::Vel;
use crate::error::LoginError;
use crate::login::login;

pub mod builder;

pub struct Server {
    listeners: Vec<Listener>,
    name: String,
    sub_name: String,
    pub world: World,
}

impl Server {
    pub async fn start(&mut self) {
        for listener in &mut self.listeners {
            listener.start().await.unwrap();
        }
    }

    pub async fn stop(&mut self) {
        for listener in &mut self.listeners {
            listener.stop().await;
        }
    }
    
    pub async fn accept<F>(&mut self, fnc: F) -> Result<(), LoginError>
    where F: FnOnce((Entity, Player, Pos, Vel)) -> Result<(Entity, Player, Pos, Vel), dyn Error> {
        let conn = self.listeners.first().unwrap().accept().await.unwrap();
        
        let player = login(conn, &mut self.world)?;
        let player = fnc(player);
        
        let id = match player {
            Ok(player) => self.world.add_entity(player),
            Err(err) => return Err(LoginError::Other(err)),
        };
        
        
        
        Ok(())
    }
}
