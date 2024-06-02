use bevy::prelude::*;
use std::ops::Range;

const SPAWN_RANGE_X: Range<f32> = -25.0 .. 25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0 .. 25.0;

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer
}

fn spawn_asteroid(mut commands: Commands, mut spawn_timer: ResMut<SpawnTimer>, time: Res<Time>, asset_server: Res<AssetServer>) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished(){
        return;
    }
}