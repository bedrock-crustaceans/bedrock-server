use bedrockrs::proto::listener::Listener;
use shipyard::World;
use crate::player::Player;

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
    
    pub async fn accept(&mut self) -> Player {
        self.listeners.first().unwrap().accept().await.unwrap();
        
        self.ecs_world.add_entity(Player {
            name: self.name.clone(),
            xuid: "".to_string(),
            cache_supported: false,
            connection: (),
        })
    }
}
