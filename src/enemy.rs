use crate::prelude::*;

#[derive(PartialEq)]
pub enum EnemyType {
    Mage(bool),
    Archer(bool),
}

#[derive(Component, Deref, DerefMut, PartialEq)]
pub struct Enemy(pub EnemyType);

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
// Above => +y or y: +400 | x can be 0 to 400
// Below => -y or y: -0 | x can be 0 to 400
// Left => -x or x: -0 | y can be 0 to 400
// Right => +x or x: +400 | y can be 0 to 400
const POS_CAP: i32 = ARENA_WORLD_SIZE as i32 + 100;
const NEG_CAP: i32 = -100;
fn spawn_initial_enemies(
    image_handles: Res<ImageHandles>,
    animation_handles: Res<AnimationHandles>,
    wambo: Res<Wambo>,
    query_enemy_node: Query<Entity, With<EnemyNode>>,
    mut commands: Commands,
) {
    let enemy_node = query_enemy_node.single();
    let mut mage_batch = Vec::new();
    for i in 0..20 {
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
        let duration = fastrand::u8(min..cap) as f32;
        let is_alt = fastrand::bool();
        let child = commands
            .spawn_bundle(mage)
            .insert(Enemy(EnemyType::Mage(is_alt)))
            .insert(Life(1))
            .insert(
                animation_handles
                    .enemy_sprite(EnemyType::Mage(is_alt))
                    .clone(),
            )
            .insert(Play)
            .insert(ShootProjectileTimer(Timer::from_seconds(duration, true)))
            .id();
        commands.entity(enemy_node).add_child(child);
    }
}

fn random_pos(i: u32) -> (f32, f32) {
    let arena_side = ARENA_WORLD_SIZE as i32;
    let (x, y) = if i <= 5 {
        // Above
        (
            fastrand::i32(0..arena_side),
            fastrand::i32(arena_side + 1..POS_CAP),
        )
    } else if i <= 10 && i > 5 {
        // Below
        (fastrand::i32(0..arena_side), fastrand::i32(NEG_CAP..-1))
    } else if i <= 15 && i > 10 {
        // Left
        (fastrand::i32(NEG_CAP..-1), fastrand::i32(0..arena_side))
    } else {
        // Right
        (
            fastrand::i32(arena_side + 1..POS_CAP),
            fastrand::i32(0..arena_side),
        )
    };

    (x as f32, y as f32)
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AssetLoad).with_system(spawn_enemy_parent))
            .add_system_set(SystemSet::on_enter(Playing).with_system(spawn_initial_enemies));
    }
}
