use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::level::Tile;

#[derive(Component)]
pub struct HoverIndicator;

pub struct HoverIndicatorPlugin;
impl Plugin for HoverIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(FixedUpdate, hover);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(0., 0.4, 0.),
            mesh: meshes.add(Cuboid::new(1.61, 0.8, 1.61)),
            material: materials.add(StandardMaterial {
                alpha_mode: AlphaMode::Blend,
                base_color: Color::hex("#00ff00c0").unwrap(),
                ..Default::default()
            }),
            visibility: Visibility::Hidden,
            ..Default::default()
        },
        HoverIndicator,
        NotShadowCaster,
    ));
}

fn hover(
    window_query: Query<&Window>,
    camera_query: Query<(&GlobalTransform, &Camera)>,
    mut indicator_query: Query<(&mut Transform, &mut Visibility), With<HoverIndicator>>,
    tile_query: Query<&Transform, (With<Tile>, Without<HoverIndicator>)>,
    rapier_context: Res<RapierContext>,
) {
    let window = window_query.single();
    let (camera_tran, camera) = camera_query.single();
    let (mut indicator_tran, mut indicator_vis) = indicator_query.single_mut();

    if let Some(coords) = window.cursor_position() {
        let camera_ray = camera.viewport_to_world(camera_tran, coords).unwrap();
        if let Some((entity, _)) = rapier_context.cast_ray(
            camera_ray.origin,
            *camera_ray.direction,
            1000.,
            true,
            QueryFilter::default(),
        ) {
            if let Ok(tile_tran) = tile_query.get(entity) {
                *indicator_vis = Visibility::Visible;
                indicator_tran.translation = tile_tran.translation + Vec3::new(0., 0.4, 0.);
            } else {
                *indicator_vis = Visibility::Hidden;
            }
        } else {
            *indicator_vis = Visibility::Hidden;
        }
    } else {
        *indicator_vis = Visibility::Hidden;
    }
}
