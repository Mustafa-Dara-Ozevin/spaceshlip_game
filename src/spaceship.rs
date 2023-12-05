use crate::{movement::{Acceleration, MovingObjectBundle, Velocity}, asset_loader::SceneAssets};
use bevy::prelude::*;
pub struct SpaceshipPlugin;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship);
    }
}

fn spawn_spaceship(mut commands: Commands, asset_scenes: Res<SceneAssets>) {
    commands.spawn(MovingObjectBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        acceleration: Acceleration::new(Vec3::ZERO),

        model: SceneBundle {
            scene: asset_scenes.spaceship.clone(),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        },
    });
}
