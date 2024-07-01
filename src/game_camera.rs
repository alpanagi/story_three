use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub struct GameCameraPlugin;
impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(TransformBundle {
            local: Transform::from_xyz(0.4 + 8. * 1.7, 0., -0.4 + 8. * 1.7),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                transform: Transform::from_xyz(-32.0, 32.0, 32.0).looking_at(Vec3::ZERO, Vec3::Y),
                camera: Camera {
                    clear_color: ClearColorConfig::Custom(Color::BLACK),
                    ..Default::default()
                },
                projection: Projection::Orthographic(OrthographicProjection {
                    scaling_mode: ScalingMode::FixedVertical(24.),
                    ..Default::default()
                }),
                ..Default::default()
            });
        });
}
