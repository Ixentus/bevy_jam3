#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use std::time::Duration;

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowPlugin};
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};

use colors::*;

pub mod colors;

#[derive(Clone, PartialEq, Eq, Default, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Splash,
    InGame,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy_jam3".to_string(),
                present_mode: PresentMode::AutoNoVsync,
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..Default::default()
        }))
        .add_plugin(FramepacePlugin)
        .insert_resource(FramepaceSettings {
            limiter: Limiter::Auto,
        })
        .insert_resource(ClearColor(BASE))
        .add_state::<AppState>()
        .add_startup_system(setup)
        .add_system(splash.in_set(OnUpdate(AppState::Splash)))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("icon.png"),
        ..Default::default()
    });
}

fn splash(
    mut commands: Commands,
    mut state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    icon: Query<Entity, With<Sprite>>,
) {
    if time.startup().elapsed() > Duration::new(2, 0) {
        state.set(AppState::InGame);
        commands.entity(icon.single()).despawn();
    }
}
