use bevy::prelude::*;
use game_display::{GameDisplayPlugin, MyGame};

mod common;
mod game_display;
mod logic;

fn main() {
    App::new()
        .insert_resource(MyGame::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(GameDisplayPlugin)
        .run();
}
