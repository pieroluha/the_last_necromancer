use crate::prelude::*;
use bevy::render::{render_resource::WgpuFeatures, settings::WgpuSettings};

#[derive(PartialEq)]
pub enum ProjectileType {
    Arrow,
    Fireball,
    Laser,
}

#[derive(Component)]
pub struct Projectile(ProjectileType);

//#[derive(Component, Deref, DerefMut)]
//pub struct DespawnTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct ShootProjectileTimer(pub Timer);

#[derive(Component)]
struct ProjectileNode;

fn setup_projectile_parent(mut commands: Commands) {
    commands
        .spawn()
        .insert(GlobalTransform::default())
        .insert(Transform::default())
        .insert(ProjectileNode)
        .insert(Name::new("ProjectileNode"));
}

pub const FIREBALL_SPEED: f32 = 200.0;
pub const MULTIPLIER: f32 = 3.0;

fn enemy_shoot_projectile(
    time: Res<Time>,
    image_handles: Res<ImageHandles>,
    animation_handles: Res<AnimationHandles>,
    query_node: Query<Entity, With<ProjectileNode>>,
    mut query_enemies: Query<(&Transform, &mut ShootProjectileTimer, &Enemy)>,
    mut commands: Commands,
) {
    for (enemy, mut timer, enemy_type) in query_enemies.iter_mut() {
        timer.tick(time.delta());

        if timer.just_finished() {
            let parent_node = query_node.single();

            let (projectile_type, image_handle, animation_handle) = match enemy_type.0 {
                EnemyType::Mage(true) | EnemyType::Mage(false) => (
                    ProjectileType::Fireball,
                    image_handles.fireball.clone(),
                    animation_handles.fireball.clone(),
                ),
                EnemyType::Archer(true) | EnemyType::Archer(false) => (
                    ProjectileType::Fireball,
                    image_handles.fireball.clone(),
                    animation_handles.fireball.clone(),
                ),
            };

            let child = commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: image_handle,
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::new(16.0, 16.0)),
                        ..default()
                    },
                    transform: *enemy,
                    ..default()
                })
                .insert(animation_handle)
                .insert(Play)
                .insert(Velocity::from_linear(look_at_player(&enemy.translation)))
                .insert(RigidBody::KinematicPositionBased)
                .insert(CollisionShape::Cuboid {
                    half_extends: Vec2::splat(16.0 / 2.0).extend(0.0),
                    border_radius: None,
                })
                .insert(
                    CollisionLayers::none()
                        .with_group(EntityLayer::Projectile)
                        .with_masks(&PROJECTILE_MASK),
                )
                .insert(Speed(FIREBALL_SPEED))
                .insert(Projectile(projectile_type))
                //.insert(DespawnTimer(Timer::from_seconds(5.0, false)))
                .id();

            commands.entity(parent_node).add_child(child);
        }
    }
}

fn move_projectiles(
    time: Res<Time>,
    mut query_projectiles: Query<(&mut Transform, &Velocity, &Speed, &Projectile)>,
) {
    for (mut transform, velocity, speed, projectile_type) in query_projectiles.iter_mut() {
        transform.translation += velocity.linear * time.delta_seconds() * speed.0;
    }
}

//fn despawn_timer(
//    time: Res<Time>,
//    mut commands: Commands,
//    mut query_projectiles: Query<(&mut DespawnTimer, Entity), With<Projectile>>,
//) {
//    for (mut timer, projectile) in query_projectiles.iter_mut() {
//        if timer.tick(time.delta()).just_finished() {
//            commands.entity(projectile).despawn_recursive();
//            println!("Projectile despawned: {}", projectile.id());
//        }
//    }
//}

pub struct ProjectilesPlugin;
impl Plugin for ProjectilesPlugin {
    fn build(&self, app: &mut App) {
        let mut options = WgpuSettings::default();
        options
            .features
            .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);

        app.add_system_set(SystemSet::on_enter(AssetLoad).with_system(setup_projectile_parent))
            .add_system_set(SystemSet::on_update(Playing).with_system(enemy_shoot_projectile))
            .add_system_set(SystemSet::on_update(Playing).with_system(move_projectiles));
        //.add_system_set(SystemSet::on_update(Playing).with_system(despawn_timer));
    }
}
