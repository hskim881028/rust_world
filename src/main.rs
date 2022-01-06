use bevy::prelude::*;

mod player;
use crate::player::PlayerPlugin;

fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(PlayerPlugin)    
    .run();
}