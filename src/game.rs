use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct Game {
    pub panel: Vec<Vec<u8>>,
    pub score: u32,
    pub steps: u32,
}
