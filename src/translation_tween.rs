use bevy::prelude::*;

const MOVEMENT_SPEED: f32 = 8.;

#[derive(Component)]
pub struct TranslationTween {
    pub target: Vec3,
}

pub struct TranslationTweenPlugin;
impl Plugin for TranslationTweenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick);
    }
}

fn tick(
    mut tween_query: Query<(Entity, &mut Transform, &TranslationTween)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut tran, tween) in tween_query.iter_mut() {
        if tran.translation.distance(tween.target) < 0.1 {
            tran.translation = tween.target;
            commands.entity(entity).remove::<TranslationTween>();
        } else {
            tran.translation = tran.translation
                + (tween.target - tran.translation).normalize()
                    * MOVEMENT_SPEED
                    * time.delta_seconds();
        }
    }
}
