use shipyard::Component;

pub mod player;
pub mod position;
pub mod velocity;

#[derive(Component)]
pub struct Entity {
    name: String
}

impl Entity {
    pub fn new(name: String) -> Self {
        Entity {
            name
        }
    }
}
