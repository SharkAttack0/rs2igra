use bevy::prelude::*;

#[derive(SystemSet, Hash, PartialEq, Eq, Clone, Debug)]
pub enum InGameSet {
    UserInput,
    EntityUpdates,
    CollisionDetection,
    DespawnEntities,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InGameSet::DespawnEntities,
                InGameSet::UserInput,
                InGameSet::CollisionDetection,
                InGameSet::EntityUpdates,
            )
                .chain(),
        )
        //NOTE: APPARENTLY, THIS IS DONE AUTOMATICALLY SINCE 0.13
        //(0.12 DOESN'T AUTOMATE IT)
        .add_systems(
            Update,
            apply_deferred
                .after(InGameSet::DespawnEntities)
                .before(InGameSet::UserInput),
        );
    }
}
