use crate::prelude::*;
//use bevy::render::{render_resource::WgpuFeatures, settings::WgpuSettings};

#[derive(PartialEq, Clone, Copy)]
pub enum ProjectileType {
    Arrow,
    Fireball,
    Special,
}

#[derive(Component, Clone, Copy)]
pub struct Projectile(pub ProjectileType);

#[derive(Component, Deref, DerefMut)]
pub struct DespawnTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct ShootProjectileTimer(pub Timer);

#[derive(Component)]
struct ProjectileNode;

#[derive(Component)]
pub struct UltimaNode;

fn setup_projectile_parent(mut commands: Commands) {
    commands
        .spawn()
        .insert(GlobalTransform::default())
        .insert(Transform::default())
        .insert(ProjectileNode)
        .insert(Name::new("ProjectileNode"));
}

fn setup_ultima_parent(mut commands: Commands) {
    commands
        .spawn()
        .insert(GlobalTransform::default())
        .insert(Transform::default())
        .insert(UltimaNode)
        .insert(Name::new("UltimaParent"));
}

pub const BOLT_BASE_SPEED: f32 = 50.0;
pub const ARROW_BASE_SPEED: f32 = 80.0;
//pub const MULTIPLIER: f32 = 2.0;

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

            let (
                projectile_type,
                image_handle,
                animation_handle,
                sprite_size,
                base_speed,
                transform,
            ) = match enemy_type.0 {
                EnemyType::Mage(true) | EnemyType::Mage(false) => (
                    ProjectileType::Fireball,
                    image_handles.fireball.clone(),
                    animation_handles.fireball.clone(),
                    Vec2::new(16.0, 16.0),
                    BOLT_BASE_SPEED,
                    *enemy,
                ),
                EnemyType::Archer(true) | EnemyType::Archer(false) => (
                    ProjectileType::Arrow,
                    image_handles.arrow.clone(),
                    animation_handles.arrow_fly.clone(),
                    Vec2::new(13.0, 16.0),
                    ARROW_BASE_SPEED,
                    heading(&*enemy, 90.0),
                ),
            };

            let child = commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: image_handle,
                    sprite: TextureAtlasSprite {
                        custom_size: Some(sprite_size),
                        ..default()
                    },
                    transform,
                    ..default()
                })
                .insert(animation_handle)
                .insert(Play)
                .insert(Velocity::from_linear(look_at_player(&enemy.translation)))
                .insert(RigidBody::KinematicPositionBased)
                .insert(CollisionShape::Cuboid {
                    half_extends: ((sprite_size - 1.0) / 2.0).extend(0.0),
                    border_radius: None,
                })
                .insert(
                    CollisionLayers::none()
                        .with_group(EntityLayer::Projectile)
                        .with_masks(&PROJECTILE_MASK),
                )
                .insert(Speed(base_speed))
                .insert(Projectile(projectile_type))
                .id();

            commands.entity(parent_node).add_child(child);
        }
    }
}

fn move_projectiles(
    time: Res<Time>,
    mut query_projectiles: Query<(&mut Transform, &Velocity, &Speed)>,
) {
    for (mut transform, velocity, speed) in query_projectiles.iter_mut() {
        transform.translation += velocity.linear * time.delta_seconds() * speed.0;
    }
}

fn despawn_timer(
    time: Res<Time>,
    mut commands: Commands,
    mut query_projectiles: Query<(&mut DespawnTimer, Entity), With<Projectile>>,
) {
    for (mut timer, projectile) in query_projectiles.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            commands.entity(projectile).despawn_recursive();
        }
    }
}

pub struct ProjectilesPlugin;
impl Plugin for ProjectilesPlugin {
    fn build(&self, app: &mut App) {
        //let mut options = WgpuSettings::default();
        //options
        //    .features
        //    .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);

        app.add_system_set(SystemSet::on_enter(AssetLoad).with_system(setup_projectile_parent))
            .add_system_set(SystemSet::on_update(Playing).with_system(enemy_shoot_projectile))
            .add_system_set(SystemSet::on_update(Playing).with_system(despawn_timer))
            .add_system_set(SystemSet::on_enter(Playing).with_system(setup_ultima_parent))
            .add_system_set(SystemSet::on_update(Playing).with_system(move_projectiles));
    }
}
