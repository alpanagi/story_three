use bevy::prelude::*;

const MOVEMENT_SPEED: f32 = 8.;

#[derive(Component)]
pub struct TranslationTween {
    pub path: Vec<Vec3>,
    pub idx: usize,
}

impl TranslationTween {
    pub fn new(path: Vec<Vec3>) -> Self {
        TranslationTween { path, idx: 0 }
    }
}

pub struct TranslationTweenPlugin;
impl Plugin for TranslationTweenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick);
    }
}

fn tick(
    mut tween_query: Query<(Entity, &mut Transform, &mut TranslationTween)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut tran, mut tween) in tween_query.iter_mut() {
        if tween.path.len() < 2 {
            commands.entity(entity).remove::<TranslationTween>();
            continue;
        }

        if tran.translation.distance(tween.path[tween.idx + 1]) < 0.1 {
            if tween.idx == tween.path.len() - 2 {
                tran.translation = tween.path[tween.path.len() - 1];
                commands.entity(entity).remove::<TranslationTween>();
            } else {
                tween.idx += 1;
            }
        } else {
            tran.translation = tran.translation
                + (tween.path[tween.idx + 1] - tween.path[tween.idx]).normalize()
                    * MOVEMENT_SPEED
                    * time.delta_seconds();
        }
    }
}
