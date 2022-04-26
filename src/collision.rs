use crate::prelude::*;
pub use heron::prelude::CollisionLayers;
pub use heron::prelude::CollisionShape;
pub use heron::prelude::RigidBody;
pub use heron::prelude::Velocity;
use heron::prelude::*;

#[derive(PhysicsLayer)]
pub enum EntityLayer {
    Enemy,
    Player,
    Minion,
    Projectile,
    Shield,
}

pub const PROJECTILE_MASK: [EntityLayer; 3] = [
    EntityLayer::Player,
    EntityLayer::Minion,
    EntityLayer::Shield,
];

struct HitEvent(u32, ProjectileType);

const PROJECTILE_GULAG: Vec3 = const_vec3!([6969.0, 6969.0, 0.0]);

fn projectile_collisions(
    mut query_projectiles: Query<(&Projectile, Entity, &mut Transform, &mut ProjectileHitData)>,
    mut hit_writer: EventWriter<HitEvent>,
    mut collisions: EventReader<CollisionEvent>,
) {
    collisions
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|collision| {
            let (e1, e2) = collision.rigid_body_entities();
            let (l1, l2) = collision.collision_layers();
            if is_target(l1) && is_projectile(l2) {
                Some((e2, e1))
            } else if is_target(l2) && is_projectile(l1) {
                Some((e1, e2))
            } else {
                None
            }
        })
        .for_each(|(projectile, entity)| {
            for (projectile_type, match_projectile, mut transform, mut hit_data) in
                query_projectiles.iter_mut()
            {
                if projectile.id() != match_projectile.id() {
                    continue;
                }
                hit_data.position_of_hit = transform.translation;
                hit_data.hit = true;
                transform.translation = PROJECTILE_GULAG;
                hit_writer.send(HitEvent(entity.id(), projectile_type.0));
            }
        });
}

// Add animation of projectile blowing up
fn player_collision(
    mut hits: EventReader<HitEvent>,
    mut query_player: Query<(&mut Life, Entity), With<Player>>,
) {
    for hit in hits.iter() {
        let (mut life, player) = query_player.single_mut();
        if hit.0 != player.id() {
            continue;
        }
        if hit.1 == ProjectileType::Special {
            continue;
        }
        life.0 = life.saturating_sub(1);
    }
}

fn shield_collision(
    query_shield: Query<Entity, With<Shield>>,
    mut query_player: Query<&mut Mana, With<Player>>,
    mut hits: EventReader<HitEvent>,
) {
    for hit in hits.iter() {
        let shield = query_shield.single();
        let mut player_mana = query_player.single_mut();
        if hit.0 != shield.id() {
            continue;
        }
        if hit.1 == ProjectileType::Special {
            continue;
        }
        player_mana.subtract_mana(2.5);
    }
}

fn minion_collision(
    mut hits: EventReader<HitEvent>,
    mut query_entities: Query<(&mut Life, Entity, &Minion), (With<Minion>, Without<Player>)>,
) {
    for hit in hits.iter() {
        for (mut life, entity, minion_type) in query_entities.iter_mut() {
            if hit.0 != entity.id() {
                continue;
            }

            if hit.1 == ProjectileType::Special {
                continue;
            }

            let projectile_type = hit.1;

            if (*minion_type == Minion::Demon && projectile_type == ProjectileType::Arrow)
                || (*minion_type == Minion::Skeleton && projectile_type == ProjectileType::Fireball)
            {
                life.0 = life.saturating_sub(1);
            }
        }
    }
}

fn enemy_collision(
    mut hits: EventReader<HitEvent>,
    mut query_entities: Query<(&mut Life, Entity), With<Enemy>>,
) {
    for hit in hits.iter() {
        for (mut life, entity) in query_entities.iter_mut() {

            if hit.0 != entity.id() {
                continue;
            }
            if hit.1 == ProjectileType::Special {
                life.0 = life.saturating_sub(1);
            }
        }
    }
}

fn is_target(layers: CollisionLayers) -> bool {
    layers.contains_group(EntityLayer::Minion)
        || layers.contains_group(EntityLayer::Player)
        || layers.contains_group(EntityLayer::Shield)
        || layers.contains_group(EntityLayer::Enemy)
            && !layers.contains_group(EntityLayer::Projectile)
}

fn is_projectile(layers: CollisionLayers) -> bool {
    layers.contains_group(EntityLayer::Projectile)
        && !layers.contains_group(EntityLayer::Minion)
        && !layers.contains_group(EntityLayer::Player)
        && !layers.contains_group(EntityLayer::Shield)
        && !layers.contains_group(EntityLayer::Enemy)
}

// For the selection box collision
#[derive(Component)]
pub struct RectAABB {
    pub pos: Vec2,
    pub size: Vec2,
}

impl Default for RectAABB {
    fn default() -> Self {
        Self {
            pos: Vec2::ZERO,
            size: Vec2::ZERO,
        }
    }
}

impl RectAABB {
    pub fn collision_check(&self, other: &RectAABB) -> bool {
        let a = self.pos;
        let asi = self.size;
        let b = other.pos;
        let bsi = other.size;

        (a.x - b.x).abs() * 2.0 < (asi.x + bsi.x) && (a.y - b.y).abs() * 2.0 < (asi.y + bsi.y)
    }
}

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PhysicsPlugin::default())
            .add_event::<HitEvent>()
            .add_system_set(SystemSet::on_update(Playing).with_system(projectile_collisions))
            .add_system_set(SystemSet::on_update(Playing).with_system(minion_collision))
            .add_system_set(SystemSet::on_update(Playing).with_system(enemy_collision))
            .add_system_set(SystemSet::on_update(Playing).with_system(shield_collision))
            .add_system_set(SystemSet::on_update(Playing).with_system(player_collision));
    }
}
