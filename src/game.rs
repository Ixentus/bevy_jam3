use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, -Vec3::Z),
        projection: OrthographicProjection::default().into(),
        ..default()
    });
}
