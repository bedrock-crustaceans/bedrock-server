use bedrockrs::proto::connection::ConnectionShard;
use shipyard::Component;
use xuid::Xuid;

#[derive(Component)]
pub struct Player {
    xuid: Option<Xuid>,
    connection: ConnectionShard,
}

impl Player {
    pub fn new(xuid: Option<Xuid>, connection: ConnectionShard) -> Self {
        Self {
            xuid,
            connection
        }
    }
}
