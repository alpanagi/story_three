use bevy::prelude::*;

pub struct GameCameraPlugin;
impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(TransformBundle {
            local: Transform::from_rotation(Quat::from_rotation_y(-45_f32.to_radians()))
                .with_translation(Vec3::new(10.6, 0., 16.6)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                transform: Transform::from_xyz(0., 40., 40.)
                    .with_rotation(Quat::from_rotation_x(-45_f32.to_radians())),
                camera: Camera {
                    clear_color: ClearColorConfig::Custom(Color::hex("#172038").unwrap()),
                    ..Default::default()
                },
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: 27_f32.to_radians(),
                    near: 0.1,
                    far: 10000.,
                    ..Default::default()
                }),
                ..Default::default()
            });
        });
}
