use std::time::Duration;

use bevy::prelude::*;

use crate::AppState;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("icon.png"),
        ..Default::default()
    });
}

pub fn splash(
    mut commands: Commands,
    mut state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    icon: Query<Entity, With<Sprite>>,
    camera: Query<Entity, With<Camera>>,
) {
    if time.startup().elapsed() > Duration::new(1, 0) {
        state.set(AppState::InGame);
        commands.entity(icon.single()).despawn();
        commands.entity(camera.single()).despawn();
    }
}
