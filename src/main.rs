use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};

use environment::EnvironmentPlugin;
use game_camera::GameCameraPlugin;
use hover_indicator::HoverIndicatorPlugin;
use instructions_screen::InstructionsScreenPlugin;
use level::LevelPlugin;
use player::PlayerPlugin;
use success_screen::SuccessScreenPlugin;
use translation_tween::TranslationTweenPlugin;

mod a_star;
mod environment;
mod game_camera;
mod game_state;
mod hover_indicator;
mod instructions_screen;
mod level;
mod player;
mod success_screen;
mod translation_tween;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins((
            EnvironmentPlugin,
            GameCameraPlugin,
            HoverIndicatorPlugin,
            InstructionsScreenPlugin,
            LevelPlugin,
            PlayerPlugin,
            SuccessScreenPlugin,
            TranslationTweenPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut window_query: Query<&mut Window>) {
    let mut window = window_query.single_mut();
    window.resolution.set(960., 540.);
}
