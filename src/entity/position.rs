use shipyard::Component;

#[derive(Component)]
pub struct Pos {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Component)]
pub struct Vel {
    x: f32,
    y: f32,
    z: f32,
}

