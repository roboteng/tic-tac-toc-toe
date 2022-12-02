use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use game_display::*;
use logic::*;

mod common;
mod game_display;
mod logic;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .insert_resource({
            let mut b = Board::new();
            b.place(Player::B, 0, 2, 3).unwrap();
            b.place(Player::A, 0, 0, 0).unwrap();
            b.place(Player::A, 3, 3, 3).unwrap();
            MyBoard { board: b }
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(create_frame)
        .add_startup_system(make_selector)
        .add_system(replace_board)
        .add_system(pulse_selector)
        .add_system(handle_input)
        .run();
}
