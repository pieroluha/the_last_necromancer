use crate::prelude::*;

// Demon's are immune to magic damage but not arrow damage
// Skeleton's are immune to arrow damage but not magic damage
#[derive(Component, PartialEq)]
pub enum Minion {
    Demon,
    Skeleton,
}

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum AnimState {
    Idle,
    Run,
}

#[derive(Component, PartialEq, Eq)]
pub struct OldState(pub AnimState);
impl Default for OldState {
    fn default() -> Self {
        Self(AnimState::Idle)
    }
}

#[derive(Component)]
pub struct MinionNode;

fn spawn_minion_parent(mut commands: Commands) {
    commands
        .spawn()
        .insert(GlobalTransform::default())
        .insert(Transform::default())
        .insert(Name::new("MinionNode"))
        .insert(MinionNode);
}

fn spawn_initial_minions(
    image_handles: Res<ImageHandles>,
    animation_handles: Res<AnimationHandles>,
    query_minion_node: Query<Entity, With<MinionNode>>,
    mut commands: Commands,
) {
    let parent_node = query_minion_node.single();

    let positions = [
        Vec2::new(152.0, 160.0),
        Vec2::new(152.0, 256.0),
        Vec2::new(248.0, 258.0),
        Vec2::new(248.0, 160.0),
        Vec2::new(248.0, 160.0),
        Vec2::new(248.0, 160.0),
        Vec2::new(248.0, 160.0),
        Vec2::new(248.0, 160.0),
        Vec2::new(248.0, 160.0),
        Vec2::new(248.0, 160.0),
        Vec2::new(248.0, 160.0),
        Vec2::new(248.0, 160.0),
    ];

    let mut demon_batch = Vec::new();
    for pos in positions.into_iter() {
        demon_batch.push(SpriteSheetBundle {
            texture_atlas: image_handles.demon.clone(),
            transform: Transform::from_translation(pos.extend(1.0)),
            ..default()
        });
    }

    for demon in demon_batch.into_iter() {
        let child = commands
            .spawn_bundle(demon)
            .insert(Minion::Demon)
            .insert(AnimState::Idle)
            .insert(OldState::default())
            .insert(SelectedUnit::default())
            .insert(animation_handles.demon_idle.clone())
            .insert(Play)
            .insert(Life(5))
            .insert(RigidBody::KinematicPositionBased)
            .insert(CollisionShape::Cuboid {
                half_extends: (Vec2::new(16.0, 24.0) / 2.0).extend(0.0),
                border_radius: None,
            })
            .insert(CollisionLayers::new(
                EntityLayer::Minion,
                EntityLayer::Projectile,
            ))
            .insert(RectAABB {
                pos: Vec2::ZERO,
                size: Vec2::new(16.0, 24.0),
            })
            .id();

        commands.entity(parent_node).add_child(child);
    }
}

fn monitor_minion_anim_state(
    animation_handles: Res<AnimationHandles>,
    mut query_minion: Query<(&AnimState, &mut OldState, Entity), With<Minion>>,
    mut commands: Commands,
) {
    for (anim_state, mut old_state, minion) in query_minion.iter_mut() {
        if old_state.0 == *anim_state {
            continue;
        }
        if *anim_state == AnimState::Idle {
            commands
                .entity(minion)
                .insert(animation_handles.demon_idle.clone());
        } else {
            commands
                .entity(minion)
                .insert(animation_handles.demon_run.clone());
        }

        old_state.0 = anim_state.clone();
    }
}

fn update_rect_aabb(mut query_minion: Query<(&Transform, &mut RectAABB), With<Minion>>) {
    for (transform, mut rect) in query_minion.iter_mut() {
        rect.pos = transform.translation.truncate();
    }
}

pub struct MinionPlugin;
impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AssetLoad).with_system(spawn_minion_parent))
            .add_system_set(SystemSet::on_enter(Playing).with_system(spawn_initial_minions))
            .add_system_set(SystemSet::on_update(Playing).with_system(monitor_minion_anim_state))
            .add_system_set(SystemSet::on_update(Playing).with_system(update_rect_aabb));
    }
}
