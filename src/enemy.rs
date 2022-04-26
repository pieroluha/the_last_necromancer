use crate::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum EnemyType {
    Mage(bool),
    Archer(bool),
}

#[derive(Component, Deref, DerefMut, PartialEq)]
pub struct Enemy(pub EnemyType);

pub struct EnemyCount {
    pub current: u8,
    pub mages: u8,
    pub archers: u8,
}
impl Default for EnemyCount {
    fn default() -> Self {
        Self {
            current: 15,
            mages: 100,
            archers: 100,
        }
    }
}

#[derive(Component)]
pub struct EnemyNode;

fn spawn_enemy_parent(mut commands: Commands) {
    commands
        .spawn()
        .insert(GlobalTransform::default())
        .insert(Transform::default())
        .insert(Name::new("EnemyNode"))
        .insert(EnemyNode);
}

// Boundaries
// Upper left corner 80, 512
// Upper right corner 512, 512
// Lower left 80, 80
// Lower right 512, 80
// cap 80 512
// 48 or 64 for upper area
pub const INNER_MAP_MAX: i32 = 528;
pub const INNER_MAP_MIN: i32 = 64;
pub const POS_CAP: i32 = INNER_MAP_MAX + 32;
pub const NEG_CAP: i32 = INNER_MAP_MIN - 32;
const ENEMY_COUNT: u32 = 16;
fn spawn_initial_enemies(
    image_handles: Res<ImageHandles>,
    animation_handles: Res<AnimationHandles>,
    wambo: Res<Wambo>,
    query_enemy_node: Query<Entity, With<EnemyNode>>,
    enemy_count: Res<EnemyCount>,
    mut commands: Commands,
) {
    let enemy_node = query_enemy_node.single();
    let mut mage_batch = Vec::new();

    for i in 0..ENEMY_COUNT {
        let (x, y) = random_pos(i);
        mage_batch.push(SpriteSheetBundle {
            texture_atlas: image_handles.enemies.clone(),
            transform: Transform::from_xyz(x, y, 1.0),
            ..default()
        })
    }

    let min: u8 = 3 - wambo.0;
    let cap: u8 = 6 - wambo.0;
    for mage in mage_batch.into_iter() {
        let enemy_type = get_enemy_type(&enemy_count);
        let duration = fastrand::u8(min..cap) as f32;
        let child = commands
            .spawn_bundle(mage)
            .insert(Enemy(enemy_type.clone()))
            .insert(Life(1))
            .insert(animation_handles.enemy_sprite(enemy_type).clone())
            .insert(Play)
            .insert(RigidBody::KinematicPositionBased)
            .insert(CollisionShape::Cuboid {
                half_extends: Vec2::splat(16.0 * 0.5).extend(0.0),
                border_radius: None,
            })
            .insert(CollisionLayers::new(
                EntityLayer::Enemy,
                EntityLayer::Projectile,
            ))
            .insert(ShootProjectileTimer(Timer::from_seconds(duration, true)))
            .id();

        commands.entity(enemy_node).add_child(child);
    }
}

fn current_enemy_count(
    mut enemy_count: ResMut<EnemyCount>,
    image_handles: Res<ImageHandles>,
    animation_handles: Res<AnimationHandles>,
    query_enemy_node: Query<Entity, With<EnemyNode>>,
    mut commands: Commands,
) {
    let parent = query_enemy_node.single();

    if enemy_count.current != 16 {
        let i = fastrand::u32(0..16);
        let (x, y) = random_pos(i);
        enemy_count.current += 1;

        let mage = SpriteSheetBundle {
            texture_atlas: image_handles.enemies.clone(),
            transform: Transform::from_xyz(x, y, 1.0),
            ..default()
        };

        let enemy_type = get_enemy_type(&enemy_count);
        let duration = fastrand::u8(3..6) as f32;
        let child = commands
            .spawn_bundle(mage)
            .insert(Enemy(enemy_type.clone()))
            .insert(Life(1))
            .insert(animation_handles.enemy_sprite(enemy_type).clone())
            .insert(Play)
            .insert(RigidBody::KinematicPositionBased)
            .insert(CollisionShape::Cuboid {
                half_extends: Vec2::splat(16.0 * 0.5).extend(0.0),
                border_radius: None,
            })
            .insert(CollisionLayers::new(
                EntityLayer::Enemy,
                EntityLayer::Projectile,
            ))
            .insert(ShootProjectileTimer(Timer::from_seconds(duration, true)))
            .id();

        commands.entity(parent).add_child(child);
    }
}

// Create a random number from 0 to 15
fn random_pos(i: u32) -> (f32, f32) {
    let arena_side = ARENA_WORLD_SIZE as i32;
    let (x, y) = if i < 4 {
        // Above
        (
            fastrand::i32(0..arena_side),
            fastrand::i32(INNER_MAP_MAX..POS_CAP),
        )
    } else if i < 8 && i >= 4 {
        // Below
        (
            fastrand::i32(0..arena_side),
            fastrand::i32(NEG_CAP..INNER_MAP_MIN),
        )
    } else if i < 12 && i >= 8 {
        // Left
        (
            fastrand::i32(NEG_CAP..INNER_MAP_MIN),
            fastrand::i32(0..arena_side),
        )
    } else {
        // Right
        (
            fastrand::i32(INNER_MAP_MAX..POS_CAP),
            fastrand::i32(0..arena_side),
        )
    };

    (x as f32, y as f32)
}

fn get_enemy_type(enemy_count: &EnemyCount) -> EnemyType {
    let enemy_type = if enemy_count.mages != 0 && enemy_count.archers != 0 {
        if fastrand::bool() {
            EnemyType::Mage(fastrand::bool())
        } else {
            EnemyType::Archer(fastrand::bool())
        }
    } else if enemy_count.mages == 0 {
        EnemyType::Archer(fastrand::bool())
    } else {
        EnemyType::Mage(fastrand::bool())
    };

    enemy_type
}

pub struct EnemyTeleportTimer {
    pub main_timer: Timer,
}

impl Default for EnemyTeleportTimer {
    fn default() -> Self {
        Self {
            main_timer: Timer::from_seconds(12.0, true),
        }
    }
}

pub struct TeleportEvent;
fn teleport_timer(
    time: Res<Time>,
    mut countdown: ResMut<EnemyTeleportTimer>,
    mut event_writer: EventWriter<TeleportEvent>,
) {
    countdown.main_timer.tick(time.delta());

    if countdown.main_timer.just_finished() {
        event_writer.send(TeleportEvent);
    }
}

#[derive(Default)]
struct ArenaDirection {
    above: u8,
    below: u8,
    left: u8,
    right: u8,
}

fn teleport_enemy(
    mut event_reader: EventReader<TeleportEvent>,
    mut query_enemy: Query<&mut Transform, With<Enemy>>,
) {
    for _teleport_event in event_reader.iter() {
        let mut arena_dir = ArenaDirection::default();
        for mut enemy in query_enemy.iter_mut() {
            let pos = if arena_dir.above < 4 {
                arena_dir.above += 1;
                random_pos(0)
            } else if arena_dir.below < 4 {
                arena_dir.below += 1;
                random_pos(4)
            } else if arena_dir.left < 4 {
                arena_dir.left += 1;
                random_pos(8)
            } else {
                arena_dir.right += 1;
                random_pos(12)
            };

            enemy.translation = Vec2::new(pos.0, pos.1).extend(1.0);
        }
    }
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TeleportEvent>()
            .init_resource::<EnemyTeleportTimer>()
            .init_resource::<EnemyCount>()
            .add_system_set(SystemSet::on_update(Playing).with_system(current_enemy_count))
            .add_system_set(SystemSet::on_enter(AssetLoad).with_system(spawn_enemy_parent))
            .add_system_set(SystemSet::on_enter(Playing).with_system(spawn_initial_enemies))
            .add_system_set(SystemSet::on_update(Playing).with_system(teleport_timer))
            .add_system_set(SystemSet::on_update(Playing).with_system(teleport_enemy));
    }
}
