use bevy::prelude::*;

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, main_camera_startup_system);
    }
}

#[derive(Component)]
pub struct MainCamera(pub usize);

fn main_camera_startup_system(mut cmd: Commands) {
    cmd.spawn((
        Name::new(format!("Main Camera {}", 0)),
        Camera2dBundle {
            transform: Transform::from_xyz(0., 0., 1000.),
            projection: OrthographicProjection {
                near: -1000.,
                far: 1000.,
                ..Default::default()
            },
            ..Default::default()
        },
        MainCamera(0),
    ));
}
