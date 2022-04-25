use crate::prelude::*;
use bevy::math::const_vec3;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Shield(bool);
impl Shield {
    fn toggle(&mut self) {
        if self.0 == true {
            self.0 = false;
        } else {
            self.0 = true;
        }
    }
}

impl Mana {
    pub fn add_mana(&mut self, val: f32) {
        if self.0 < 500.0 {
            self.0 += val;
        } else {
            self.0 = 500.0
        }
    }

    pub fn subtract_mana(&mut self, val: f32) {
        self.0 -= val;
        if self.0 < 0.0 {
            self.0 = -10.0;
        }
    }
}

#[derive(Default)]
pub struct SpellProgress(pub f32);
impl SpellProgress {
    fn add_progress(&mut self, val: f32) {
        if self.0 < 100.0 {
            self.0 += val;
        } else {
            self.0 = 110.0;
        }
    }
}

pub const PLAYER_POS: Vec3 = const_vec3!([ARENA_OFFSET, ARENA_OFFSET, 2.0]);
pub const PLAYER_UP: Vec3 = const_vec3!([ARENA_OFFSET, ARENA_OFFSET + 16.0, 2.0]);

fn spawn_player(
    image_handles: Res<ImageHandles>,
    animation_handles: Res<AnimationHandles>,
    mut commands: Commands,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: image_handles.player.clone(),
            transform: Transform::from_translation(PLAYER_POS),
            ..default()
        })
        .insert(Player)
        .insert(Name::new("Player"))
        .insert(Life(20))
        .insert(Mana(500.0))
        .insert(animation_handles.idle_player.clone())
        .insert(Play)
        .insert(RigidBody::Sensor)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec2::splat(16.0 / 2.0).extend(0.0),
            border_radius: None,
        })
        .insert(CollisionLayers::new(
            EntityLayer::Player,
            EntityLayer::Projectile,
        ))
        .with_children(|p| {
            p.spawn_bundle(SpriteSheetBundle {
                texture_atlas: image_handles.shield.clone(),
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..default()
                },
                ..default()
            })
            .insert(Shield(true))
            .insert(animation_handles.idle_player.clone()) // Should have just renamed to idle_anim :(
            .insert(Play)
            .insert(RigidBody::Sensor)
            .insert(CollisionShape::Cuboid {
                half_extends: Vec2::splat(32.0 / 2.0).extend(0.0),
                border_radius: None,
            })
            .insert(CollisionLayers::new(
                EntityLayer::Shield,
                EntityLayer::Projectile,
            ));
        });
}

fn shield_toggle(
    query_action: Query<&ActionState<Action>>,
    query_player: Query<&Mana, With<Player>>,
    mut query_shield: Query<&mut Shield>,
) {
    let action = query_action.single();
    if !action.just_pressed(ShieldToggle) {
        return;
    }
    let mana = query_player.single();
    if mana.0 < 0.0 {
        return;
    }
    let mut shield = query_shield.single_mut();
    shield.toggle();
}

fn shield_state(
    mut query_shield: Query<&mut Shield>,
    mut query_player: Query<&mut Mana, With<Player>>,
    mut spell_progress: ResMut<SpellProgress>,
) {
    let mut shield = query_shield.single_mut();
    let mut mana = query_player.single_mut();

    if mana.0 < 0.0 {
        shield.0 = false
    }

    if shield.0 == false {
        mana.add_mana(0.2);
    } else {
        mana.subtract_mana(0.05);
        spell_progress.add_progress(10.0);
    }
}

const OUTSIDE: Vec3 = const_vec3!([9999.0, 9999.0, 9999.0]);
const DEFAULT: Vec3 = const_vec3!([0.0, 0.0, 3.0]);

fn shield_visibility(
    mut query_shield: Query<(&Shield, &mut Visibility, &mut Transform)>,
    mut old_state: Local<bool>,
) {
    let (shield, mut visibility, mut transform) = query_shield.single_mut();

    if *old_state == shield.0 {
        return;
    }
    *old_state = shield.0;

    if shield.0 == false {
        transform.translation = OUTSIDE;
        visibility.is_visible = false;
    } else {
        transform.translation = DEFAULT;
        visibility.is_visible = true;
    }
}

#[derive(PartialEq)]
pub enum SpellType {
    Deez,
    SkullBuster,
    DarkEdge,
}

struct UltimaEvent;
struct UltimaTimer(Timer, Timer);
impl Default for UltimaTimer {
    fn default() -> Self {
        Self(Timer::default(), Timer::default())
    }
}

fn fire_ultimate_spell(
    image_handles: Res<ImageHandles>,
    animation_handles: Res<AnimationHandles>,
    mut spell_progress: ResMut<SpellProgress>,
    mut event_writer: EventWriter<UltimaEvent>,
    mut done: Local<bool>,
    mut commands: Commands,
) {
    if *done {
        return;
    }
    if spell_progress.0 < 100.0 {
        return;
    }
    *done = true;
    spell_progress.0 = 0.0;

    commands.insert_resource(UltimaTimer(
        Timer::from_seconds(12.0, false),
        Timer::from_seconds(2.0, true),
    ));

    let dice = fastrand::u32(0..100);

    let spell_type = if dice > 30 {
        SpellType::DarkEdge
    } else if dice > 15 && dice < 30 {
        SpellType::SkullBuster
    } else {
        SpellType::Deez
    };

    let (sprite_atlas, animation_handle) = if spell_type == SpellType::DarkEdge {
        (
            image_handles.dark_edge.clone(),
            animation_handles.dark_edge.clone(),
        )
    } else if spell_type == SpellType::SkullBuster {
        (
            image_handles.skull_buster.clone(),
            animation_handles.skull_buster.clone(),
        )
    } else {
        (
            image_handles.stupid.clone(),
            animation_handles.idle_player.clone(),
        )
    };

    commands.insert_resource(CurrentSpell {
        sprite_atlas,
        animation_handle,
        spell_type,
    });

    event_writer.send(UltimaEvent);
}

struct CurrentSpell {
    sprite_atlas: Handle<TextureAtlas>,
    animation_handle: Handle<SpriteSheetAnimation>,
    spell_type: SpellType,
}

fn ultima_event(
    time: Res<Time>,
    query_ultima_node: Query<Entity, With<UltimaNode>>,
    current_spell: Option<Res<CurrentSpell>>,
    mut boom_time: Local<bool>,
    mut event_reader: EventReader<UltimaEvent>,
    mut timer: ResMut<UltimaTimer>,
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
            let parent = query_ultima_node.single();
            let sprite_size = Vec2::new(32.0, 32.0);
            let transform = Transform::from_translation(PLAYER_POS);
            let sprite_atlas = current_spell.sprite_atlas.clone();
            let animation_handle = current_spell.animation_handle.clone();
            for pos in positions {
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
                        half_extends: (sprite_size / 2.0).extend(0.0),
                        border_radius: None,
                    })
                    .insert(CollisionLayers::new(
                        EntityLayer::Projectile,
                        EntityLayer::Enemy,
                    ))
                    .insert(Speed(100.0))
                    .insert(Projectile(ProjectileType::Special))
                    .insert(DespawnTimer(Timer::from_seconds(5.0, false)))
                    .id();

                commands.entity(parent).add_child(child);
            }
        } else {
            timer.1.pause();
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpellProgress>()
            .init_resource::<UltimaTimer>()
            .add_event::<UltimaEvent>()
            .add_system_set(SystemSet::on_enter(Playing).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(Playing)
                    .with_system(shield_toggle)
                    .label("shield_toggle"),
            )
            .add_system_set(
                SystemSet::on_update(Playing)
                    .with_system(shield_state)
                    .label("shield_state")
                    .after("shield_toggle"),
            )
            .add_system_set(
                SystemSet::on_update(Playing)
                    .with_system(shield_visibility)
                    .label("shield_visibility")
                    .after("shield_state"),
            )
            .add_system_set(SystemSet::on_update(Playing).with_system(fire_ultimate_spell))
            .add_system_set(SystemSet::on_update(Playing).with_system(ultima_event));
    }
}
