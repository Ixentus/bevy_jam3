use bevy::{app::AppExit, prelude::*};

pub fn input(keyboard_input: ResMut<Input<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}
