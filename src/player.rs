use crate::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Minion;

fn spawn_player(
    image_handles: Res<ImageHandles>,
    animation_handles: Res<AnimationHandles>,
    mut commands: Commands,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: image_handles.player.clone(),
            transform: Transform::from_xyz(ARENA_OFFSET, ARENA_OFFSET, 2.0),
            ..default()
        })
        .insert(Player)
        .insert(Name::new("Player"))
        .insert(animation_handles.idle_player.clone())
        .insert(Play)
        .insert(RigidBody::Sensor)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec2::splat(32.0 / 2.0).extend(0.0),
            border_radius: None,
        })
        .insert(CollisionLayers::new(
            EntityLayer::Player,
            EntityLayer::Projectile,
        ));
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(Playing).with_system(spawn_player));
    }
}
