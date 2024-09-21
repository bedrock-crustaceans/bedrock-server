use std::error::Error;
use bedrockrs::proto::listener::Listener;
use shipyard::World;
use crate::entity::entity::Entity;
use crate::entity::player::Player;
use crate::error::LoginError;
use crate::login::login;

pub mod builder;

pub struct Server {
    listeners: Vec<Listener>,
    name: String,
    sub_name: String,
    ecs_world: World,
}

impl Server {
    pub async fn start(&mut self) {
        for listener in &mut self.listeners {
            listener.start().await.unwrap();
        }
    }

    pub async fn stop(&self) {
        unimplemented!()
    }
    
    pub async fn accept<F>(&mut self, fnc: F) -> Result<(), LoginError>
    where F: FnOnce(Player) -> Result<Player, dyn Error> {
        let conn = self.listeners.first().unwrap().accept().await.unwrap();
        
        let player = login(conn)?;
        let player = fnc(player);
        
        match player {
            Ok(player) => self.ecs_world.add_entity((
                Entity::new()
            )),
            Err(_) => {}
        }
        
        
        
        Ok(())
    }
}
