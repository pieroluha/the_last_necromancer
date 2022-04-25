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
        if self.0 < 200.0 {
            self.0 += val;
        } else {
            self.0 = 200.0
        }
    }

    pub fn subtract_mana(&mut self, val: f32) {
        self.0 -= val;
        if self.0 < 0.0 {
            self.0 = -10.0;
        }
    }
}

pub const PLAYER_POS: Vec3 = const_vec3!([ARENA_OFFSET, ARENA_OFFSET, 2.0]);

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
        .insert(Mana(100.0))
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
) {
    let mut shield = query_shield.single_mut();
    let mut mana = query_player.single_mut();

    if mana.0 < 0.0 {
        shield.0 = false
    }

    if shield.0 == false {
        mana.add_mana(0.1);
    } else {
        mana.subtract_mana(0.05);
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

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(Playing).with_system(spawn_player))
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
            );
    }
}
