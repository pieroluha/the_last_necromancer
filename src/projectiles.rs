use crate::prelude::*;
use bevy::render::{render_resource::WgpuFeatures, settings::WgpuSettings};
use bevy::sprite::MaterialMesh2dBundle;
use bevy_hanabi::*;

#[derive(PartialEq)]
pub enum LaserType {
    Holy,
    Magic,
}

#[derive(PartialEq)]
pub enum ProjectileType {
    Arrow,
    Fireball,
    Laser(LaserType),
}

#[derive(Component)]
pub struct Projectile(ProjectileType);

#[derive(Component, Deref, DerefMut)]
pub struct DespawnOnEnd(Timer);

#[derive(Deref, DerefMut)]
pub struct SpawnCountdown(Timer);
impl Default for SpawnCountdown {
    fn default() -> Self {
        Self(Timer::from_seconds(5.0, true))
    }
}

struct Effect {
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<ColorMaterial>,
    effect_handle: Handle<EffectAsset>,
    spawner: Spawner,
}

pub struct Effects {
    //arrow: Effect,
    fireball: Effect,
    //laser: Effect,
}
impl Effects {
    fn new_fireball(&self) -> ParticleEffectBundle {
        let fireball = &self.fireball;
        ParticleEffectBundle::new(fireball.effect_handle.clone()).with_spawner(fireball.spawner)
    }
}

#[derive(Component)]
struct ProjectileParent;

fn setup_effect_handles(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let fireball_spawner = Spawner::rate(30.0.into());
    let fireball_effect = effects.add(
        EffectAsset {
            name: "Fireball".into(),
            capacity: 100,
            spawner: fireball_spawner,
            ..default()
        }
        .init(PositionCircleModifier {
            radius: 0.5,
            speed: 0.1.into(),
            dimension: ShapeDimension::Surface,
            ..default()
        })
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::constant(Vec2::splat(0.02)),
        }),
    );

    let fireball = Effect {
        mesh_handle: meshes
            .add(Mesh::from(shape::Quad {
                size: Vec2::splat(1.0),
                ..default()
            }))
            .into(),
        material_handle: materials.add(ColorMaterial {
            color: Color::RED,
            ..default()
        }),
        effect_handle: fireball_effect,
        spawner: fireball_spawner,
    };

    commands.insert_resource(Effects { fireball });

    commands
        .spawn()
        .insert(GlobalTransform::default())
        .insert(Transform::default())
        .insert(ProjectileParent)
        .insert(Name::new("ProjectileParent"));
}

fn spawn_projectile_countdown(
    time: Res<Time>,
    effects: Res<Effects>,
    query_parent: Query<Entity, With<ProjectileParent>>,
    mut spawn_countdown: ResMut<SpawnCountdown>,
    mut commands: Commands,
) {
    let projectile_parent = query_parent.single();
    spawn_countdown.tick(time.delta());

    if spawn_countdown.finished() {
        let child = commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: effects.fireball.mesh_handle.clone().into(),
                material: effects.fireball.material_handle.clone(),
                ..default()
            })
            .insert(Projectile(ProjectileType::Fireball))
            .insert(DespawnOnEnd(Timer::from_seconds(20.0, false)))
            .insert(Name::new("Fireball"))
            .with_children(|p| {
                p.spawn_bundle(effects.new_fireball())
                    .insert(Name::new("fireball_effect"));
            })
            .id();

        commands.entity(projectile_parent).add_child(child);

        println!("Projectile spawned!");
    }
}

fn despawn_on_end(
    time: Res<Time>,
    mut commands: Commands,
    mut query_projectiles: Query<(&mut DespawnOnEnd, Entity), With<Projectile>>,
) {
    for (mut timer, projectile) in query_projectiles.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            commands.entity(projectile).despawn_recursive();
            println!("Projectile despawned: {}", projectile.id());
        }
    }
}

pub struct ProjectilesPlugin;
impl Plugin for ProjectilesPlugin {
    fn build(&self, app: &mut App) {
        let mut options = WgpuSettings::default();
        options
            .features
            .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);
        app.init_resource::<SpawnCountdown>()
            .add_plugin(HanabiPlugin)
            .add_system_set(SystemSet::on_enter(AssetLoad).with_system(setup_effect_handles))
            .add_system_set(SystemSet::on_update(AssetLoad).with_system(spawn_projectile_countdown))
            .add_system_set(SystemSet::on_update(AssetLoad).with_system(despawn_on_end));
    }
}
