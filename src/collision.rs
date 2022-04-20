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

fn projectile_collisions(mut commands: Commands, mut collisions: EventReader<CollisionEvent>) {
    collisions
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|collision| {
            let (e1, e2) = collision.rigid_body_entities();
            let (l1, l2) = collision.collision_layers();
            if is_target(l1) && is_projectile(l2) {
                Some(e2)
            } else if is_target(l2) && is_projectile(l1) {
                Some(e1)
            } else {
                None
            }
        })
        .for_each(|projectile| commands.entity(projectile).despawn_recursive());
}

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
            .add_system_set(SystemSet::on_update(Playing).with_system(projectile_collisions));
    }
}
