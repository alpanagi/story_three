use bevy::prelude::*;

use crate::{game_state::GameState, hover_indicator::HoverIndicator};
use crate::a_star::AStar;
use crate::level::Tile;
use crate::translation_tween::TranslationTween;

#[derive(Component)]
struct Player;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::Instructions)
            .add_systems(Startup, setup)
            .add_systems(Update, movement);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("meshes/player.glb#Scene0"),
            transform: Transform::from_xyz(-2. * 1.7, 0., 0.),
            ..Default::default()
        },
        Player,
    ));
}

fn movement(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    indicator_query: Query<(&Visibility, &Transform), (With<HoverIndicator>, Without<Player>)>,
    tiles_query: Query<&Transform, With<Tile>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let (player_entity, player_tran) = player_query.single_mut();
        let (indicator_vis, indicator_tran) = indicator_query.single();

        if indicator_vis == Visibility::Visible {
            let tiles = tiles_query.iter().map(|x| x.translation).collect();
            let a_star = AStar::new(tiles);
            if let Some(path) = a_star.path(player_tran.translation, indicator_tran.translation) {
                commands
                    .entity(player_entity)
                    .insert(TranslationTween::new(path));
            }
        }
    }
}
