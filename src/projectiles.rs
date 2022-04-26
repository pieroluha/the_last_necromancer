use crate::prelude::*;
//use bevy::render::{render_resource::WgpuFeatures, settings::WgpuSettings};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ProjectileType {
    Arrow,
    Fireball,
    Special,
}

#[derive(Component, Clone, Copy)]
pub struct Projectile(pub ProjectileType);

#[derive(Component, Deref, DerefMut)]
pub struct DespawnTimer(pub Timer);
impl Default for DespawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(6.0, false))
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct ShootProjectileTimer(pub Timer);

#[derive(Component)]
pub struct ProjectileHitData {
    pub hit: bool,
    pub position_of_hit: Vec3,
}
impl ProjectileHitData {
    pub fn new(pos: Vec3) -> Self {
        Self {
            hit: false,
            position_of_hit: pos,
        }
    }
}

#[derive(Component)]
struct ProjectileNode;

#[derive(Component)]
pub struct BonkNode;

#[derive(Component)]
pub struct DababyNode;

fn setup_projectile_parent(mut commands: Commands) {
    commands
        .spawn()
        .insert(GlobalTransform::default())
        .insert(Transform::default())
        .insert(ProjectileNode)
        .insert(Name::new("ProjectileNode"));
}

fn setup_bonk_parent(mut commands: Commands) {
    commands
        .spawn()
        .insert(GlobalTransform::default())
        .insert(Transform::default())
        .insert(BonkNode)
        .insert(Name::new("BonkNode"));
}

fn setup_dababy_parent(mut commands: Commands) {
    commands
        .spawn()
        .insert(GlobalTransform::default())
        .insert(Transform::default())
        .insert(DababyNode)
        .insert(Name::new("DababyNode"));
}

pub const BOLT_BASE_SPEED: f32 = 50.0;
pub const ARROW_BASE_SPEED: f32 = 80.0;

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

            // Actual bruh moment
            let (
                projectile_type,
                (image_handle, sprite_size, animation_handle),
                base_speed,
                transform,
            ) = match enemy_type.0 {
                EnemyType::Mage(true) | EnemyType::Mage(false) => (
                    ProjectileType::Fireball,
                    image_handles.get_projectile(&animation_handles),
                    BOLT_BASE_SPEED,
                    heading(&*enemy, 0.0),
                ),
                EnemyType::Archer(true) | EnemyType::Archer(false) => (
                    ProjectileType::Arrow,
                    (
                        image_handles.arrow.clone(),
                        Vec2::new(13.0, 16.0),
                        animation_handles.arrow_fly.clone(),
                    ),
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
                    half_extends: ((sprite_size) / 2.0).extend(2.0),
                    border_radius: None,
                })
                .insert(
                    CollisionLayers::none()
                        .with_group(EntityLayer::Projectile)
                        .with_masks(&PROJECTILE_MASK),
                )
                .insert(Speed(base_speed))
                .insert(Projectile(projectile_type))
                .insert(ProjectileHitData::new(transform.translation))
                .insert(DespawnTimer::default())
                .id();

            commands.entity(parent_node).add_child(child);
        }
    }
}

pub struct CurrentSpell {
    pub sprite_atlas: Handle<TextureAtlas>,
    pub animation_handle: Handle<SpriteSheetAnimation>,
}

fn bonk_projectiles(
    time: Res<Time>,
    query_bonk_node: Query<Entity, With<BonkNode>>,
    current_spell: Option<Res<CurrentSpell>>,
    mut boom_time: Local<bool>,
    mut event_reader: EventReader<BonkEvent>,
    mut timer: ResMut<BonkTimer>,
    mut commands: Commands,
) {
    for _event in event_reader.iter() {
        *boom_time = true;
    }

    if !*boom_time {
        return;
    }
    let current_spell = if let Some(s) = current_spell {
        s
    } else {
        return;
    };

    timer.0.tick(time.delta());
    if timer.1.tick(time.delta()).just_finished() {
        if !timer.0.finished() {
            let positions = spin_me_right_round();
            let parent = query_bonk_node.single();
            let sprite_size = Vec2::new(64.0, 64.0);
            let transform = Transform::from_translation(PLAYER_POS);
            let sprite_atlas = current_spell.sprite_atlas.clone();
            let animation_handle = current_spell.animation_handle.clone();
            for pos in positions {
                println!("{}", look_at(&PLAYER_POS, &pos));
                let child = commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: sprite_atlas.clone(),
                        sprite: TextureAtlasSprite {
                            custom_size: Some(sprite_size),
                            ..default()
                        },
                        transform,
                        ..default()
                    })
                    .insert(animation_handle.clone())
                    .insert(Play)
                    .insert(Velocity::from_linear(look_at(&PLAYER_POS, &pos)))
                    .insert(RigidBody::KinematicPositionBased)
                    .insert(CollisionShape::Cuboid {
                        half_extends: (sprite_size * 0.5).extend(0.0),
                        border_radius: None,
                    })
                    .insert(CollisionLayers::new(
                        EntityLayer::Projectile,
                        EntityLayer::Enemy,
                    ))
                    .insert(Speed(100.0))
                    .insert(Projectile(ProjectileType::Special))
                    .insert(ProjectileHitData::new(Vec3::ZERO))
                    .insert(DespawnTimer::default())
                    .id();

                commands.entity(parent).add_child(child);
            }
        } else {
            *boom_time = false;
            timer.1.pause();
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

fn projectile_hits(
    mut commands: Commands,
    image_handles: Res<ImageHandles>,
    animation_handles: Res<AnimationHandles>,
    query_dababy: Query<Entity, With<DababyNode>>,
    query_projectiles: Query<(Entity, &ProjectileHitData, &Projectile)>,
) {
    for (projectile, hit_data, _projectile_type) in query_projectiles.iter() {
        if hit_data.hit != true {
            continue;
        }
        let parent = query_dababy.single();
        let pos = hit_data.position_of_hit.clone();
        commands.entity(projectile).despawn_recursive();

        let (image_handle, animation_handle) = image_handles.get_hit(&animation_handles);

        let child = commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: image_handle,
                transform: Transform::from_translation(pos),
                sprite: TextureAtlasSprite {
                    custom_size: Some(HIT_SIZE),
                    ..default()
                },
                ..default()
            })
            .insert(animation_handle)
            .insert(Play)
            .insert(DespawnTimer::default())
            .id();

        commands.entity(parent).add_child(child);
    }
}

fn despawn_timer(
    time: Res<Time>,
    mut commands: Commands,
    mut query_entities: Query<(&mut DespawnTimer, Entity)>,
) {
    for (mut timer, entity) in query_entities.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct ProjectilesPlugin;
impl Plugin for ProjectilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AssetLoad).with_system(setup_projectile_parent))
            .add_system_set(SystemSet::on_update(Playing).with_system(enemy_shoot_projectile))
            .add_system_set(
                SystemSet::on_update(Playing).with_system(despawn_timer.label("despawn_timer")),
            )
            .add_system_set(
                SystemSet::on_update(Playing)
                    .with_system(projectile_hits)
                    .before("despawn_timer"),
            )
            .add_system_set(SystemSet::on_update(Playing).with_system(move_projectiles))
            .add_system_set(SystemSet::on_update(Playing).with_system(bonk_projectiles))
            .add_system_set(SystemSet::on_update(Playing).with_system(projectile_hits))
            .add_system_set(SystemSet::on_enter(Playing).with_system(setup_bonk_parent))
            .add_system_set(SystemSet::on_enter(Playing).with_system(setup_dababy_parent));
    }
}
