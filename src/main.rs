use bevy::prelude::*;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;
use camera::CameraPlugin;

mod debug;
mod movement;
mod spaceship;
mod camera;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.2,0.0,0.15)))
    .insert_resource(AmbientLight {
        color: Color::default(),
        brightness: 1000.0,
    })
    .add_plugins(DefaultPlugins)
    .add_plugins(SpaceshipPlugin)
    .add_plugins(MovementPlugin)
    .add_plugins(DebugPlugin)
    .add_plugins(CameraPlugin)
    .run();
}