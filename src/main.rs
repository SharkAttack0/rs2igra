use asset_loader::AssetLoaderPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use debug::DebugPlugin;

use asteroid::AsteroidPlugin;
use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
use despawn::DespawnPlugin;
use movement::MovementPlugin;
use schedule::SchedulePlugin;
use spaceship::SpaceshipPlugin;
use state::StatePlugin;

mod asset_loader;
mod asteroid;
mod camera;
mod collision_detection;
mod debug;
mod despawn;
mod health;
mod movement;
mod schedule;
mod spaceship;
mod state;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1000.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(StatePlugin)
        .run();
    let a = 5;
    let b = a - 2;
}
