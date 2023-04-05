#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowPlugin};
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};

use colors::*;

pub mod colors;
pub mod game;
pub mod grid;
pub mod input;
pub mod splash;

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
        .add_startup_system(splash::setup)
        .add_system(splash::splash.in_set(OnUpdate(AppState::Splash)))
        .add_system(grid::setup_grid)
        .add_system(game::setup.in_schedule(OnEnter(AppState::InGame)))
        .add_system(input::input.in_set(OnUpdate(AppState::InGame)))
        .run();
}
