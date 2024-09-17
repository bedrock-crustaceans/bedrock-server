use bedrockrs::proto::connection::ConnectionShard;

pub struct Player {
    pub name: String,
    pub xuid: Option<String>,
    pub cache_supported: bool,
    pub connection: ConnectionShard,
}

impl Player {

}
