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
}

pub const PROJECTILE_MASK: [EntityLayer; 2] = [EntityLayer::Player, EntityLayer::Minion];

//fn test_collision(mut collisions: EventReader<CollisionEvent>) {
//    for collision in collisions.iter() {
//        match collision {
//            CollisionEvent::Started(e1, e2) => {
//                println!("{:?} and {:?} collided.", e1, e2);
//            }
//            CollisionEvent::Stopped(e1, e2) => {
//                println!("{:?} and {:?} stopped colliding.", e1, e2);
//            }
//        }
//    }
//}

#[derive(Deref, DerefMut)]
struct HitEvent(u32);

fn projectile_collisions(
    mut commands: Commands,
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
            commands.entity(projectile).despawn_recursive();
            hit_writer.send(HitEvent(entity.id()));
        });
    //.for_each(|projectile| commands.entity(projectile).despawn_recursive());
}

fn player_collision(
    mut commands: Commands,
    mut hits: EventReader<HitEvent>,
    mut query_player: Query<(&mut Life, Entity), With<Player>>,
) {
    for hit in hits.iter() {
        let (mut player_life, player) = query_player.single_mut();
        if hit.0 != player.id() {
            return;
        }
        player_life.0 = player_life.saturating_sub(1);
    }
}

//fn minion_collision(
//    mut commands: Commands,
//    mut hits: EventReader<HitEvent>,
//    mut query_player: Query<(&mut Life, Entity), With<Minion>>,
//) {
//}

fn is_target(layers: CollisionLayers) -> bool {
    layers.contains_group(EntityLayer::Minion)
        || layers.contains_group(EntityLayer::Player)
            && !layers.contains_group(EntityLayer::Projectile)
}

fn is_projectile(layers: CollisionLayers) -> bool {
    layers.contains_group(EntityLayer::Projectile)
        && !layers.contains_group(EntityLayer::Minion)
        && !layers.contains_group(EntityLayer::Player)
}

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PhysicsPlugin::default())
            .add_event::<HitEvent>()
            .add_system_set(SystemSet::on_update(Playing).with_system(projectile_collisions))
            .add_system_set(SystemSet::on_update(Playing).with_system(player_collision));
    }
}
