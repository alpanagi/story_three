use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game_state::GameState;

#[derive(Component)]
pub struct Tile;

#[derive(Resource)]
struct Map {
    image: Option<Handle<Image>>,
}

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map { image: None })
            .add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::Playing), spawn_map);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Map {
        image: Some(asset_server.load("textures/map.png")),
    });
}

fn spawn_map(
    mut commands: Commands,
    images: Res<Assets<Image>>,
    mut map: ResMut<Map>,
    asset_server: Res<AssetServer>,
) {
    if map.image.is_some() {
        let image = images.get(map.image.clone().unwrap()).unwrap();
        for i in 0..image.width() {
            for j in 0..image.height() {
                let image = image.clone().try_into_dynamic().unwrap().to_rgba8();
                let pixel = image.get_pixel(i, j);
                let position = Vec3::new(1.7 * i as f32, 0., 1.7 * j as f32);

                if pixel.0[0] == 255 && pixel.0[1] == 255 && pixel.0[2] == 255 {
                    commands.spawn((
                        SceneBundle {
                            scene: asset_server.load("meshes/tile.glb#Scene0"),
                            transform: Transform::from_translation(position),
                            ..Default::default()
                        },
                        Collider::cuboid(0.85, 0.001, 0.85),
                        Tile,
                    ));
                } else if pixel.0[0] == 255 && pixel.0[1] == 0 && pixel.0[2] == 0 {
                    commands.spawn((
                        SceneBundle {
                            scene: asset_server.load("meshes/wall.glb#Scene0"),
                            transform: Transform::from_translation(position),
                            ..Default::default()
                        },
                        Collider::cuboid(0.8, 0.8, 0.8),
                    ));
                }
            }
        }

        map.image = None;
    }
}
