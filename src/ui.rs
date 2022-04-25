use crate::prelude::*;

#[derive(Component)]
struct LifeCounter;
#[derive(Component)]
struct ManaCounter;

#[derive(Component)]
struct MageCounter;

#[derive(Component)]
struct ArcherCounter;

#[derive(Component)]
struct SpellCounter;

// Thank you inspector_egui for getting me there!
// I have little hours left and i've no time to understand this mess
fn generate_ui(
    mut commands: Commands,
    font_handles: Res<FontHandles>,
    image_handles: Res<ImageHandles>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("UiNode"))
        .with_children(|p| {
            // Health Counter
            p.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    //flex_direction: FlexDirection::Column,
                    //justify_content: JustifyContent::SpaceEvenly,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(5.0),
                        left: Val::Px(90.0),
                        ..default()
                    },
                    ..default()
                },
                text: Text::with_section(
                    "20",
                    TextStyle {
                        font: font_handles.dungeon_font.clone(),
                        font_size: 64.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        ..default()
                    },
                ),
                ..default()
            })
            .insert(Name::new("HealthCounter"))
            .insert(LifeCounter);

            // Health Icon
            p.spawn_bundle(ImageBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    flex_direction: FlexDirection::Column,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(5.0),
                        left: Val::Px(15.0),
                        ..default()
                    },
                    size: Size::new(Val::Px(64.0), Val::Auto),
                    ..default()
                },
                image: image_handles.full_heart.clone().into(),
                ..default()
            })
            .insert(Name::new("HeartIcon"));

            // Mana Counter
            p.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(69.0),
                        left: Val::Px(90.0),
                        ..default()
                    },
                    ..default()
                },
                text: Text::with_section(
                    "100%",
                    TextStyle {
                        font: font_handles.dungeon_font.clone(),
                        font_size: 64.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        ..default()
                    },
                ),
                ..default()
            })
            .insert(Name::new("ManaCounter"))
            .insert(ManaCounter);

            // Mana Icon
            p.spawn_bundle(ImageBundle {
                style: Style {
                    align_self: AlignSelf::FlexStart,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(69.0),
                        left: Val::Px(15.0),
                        ..default()
                    },
                    size: Size::new(Val::Px(64.0), Val::Auto),
                    ..default()
                },
                image: image_handles.full_mana.clone().into(),
                ..default()
            })
            .insert(Name::new("ManaIcon"));

            // Mages left
            p.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(149.0),
                        left: Val::Px(90.0),
                        ..default()
                    },
                    ..default()
                },
                text: Text::with_section(
                    "100",
                    TextStyle {
                        font: font_handles.dungeon_font.clone(),
                        font_size: 48.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        ..default()
                    },
                ),
                ..default()
            })
            .insert(MageCounter)
            .insert(Name::new("MageCounter"));

            // Mage Icon
            p.spawn_bundle(ImageBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    position: Rect {
                        top: Val::Px(149.0),
                        left: Val::Px(25.0),
                        ..default()
                    },
                    size: Size::new(Val::Px(48.0), Val::Auto),
                    ..default()
                },
                image: image_handles.mage_icon.clone().into(),
                ..default()
            })
            .insert(Name::new("MageIcon"));

            // Archers left
            p.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    position: Rect {
                        top: Val::Px(197.0),
                        left: Val::Px(90.0),
                        ..default()
                    },
                    ..default()
                },
                text: Text::with_section(
                    "100",
                    TextStyle {
                        font: font_handles.dungeon_font.clone(),
                        font_size: 48.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        ..default()
                    },
                ),
                ..default()
            })
            .insert(ArcherCounter)
            .insert(Name::new("ArcherCounter"));

            p.spawn_bundle(ImageBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    position: Rect {
                        top: Val::Px(197.0),
                        left: Val::Px(25.0),
                        ..default()
                    },
                    size: Size::new(Val::Px(48.0), Val::Auto),
                    ..default()
                },
                image: image_handles.archer_icon.clone().into(),
                ..default()
            })
            .insert(Name::new("ArcherIcon"));

            // Percentage left until deez nuts
            p.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    position: Rect {
                        top: Val::Px(261.0),
                        left: Val::Px(90.0),
                        ..default()
                    },
                    ..default()
                },
                text: Text::with_section(
                    "0%",
                    TextStyle {
                        font: font_handles.dungeon_font.clone(),
                        font_size: 48.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        ..default()
                    },
                ),
                ..default()
            })
            .insert(Name::new("SpellCounter"));

            // Spell icon
            p.spawn_bundle(ImageBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    position: Rect {
                        top: Val::Px(261.0),
                        left: Val::Px(25.0),
                        ..default()
                    },
                    size: Size::new(Val::Px(48.0), Val::Auto),
                    ..default()
                },
                image: image_handles.spell_icon.clone().into(),
                ..default()
            })
            .insert(Name::new("SpellIcon"));
        });
}

fn update_mana_counter(
    query_player: Query<&Mana, With<Player>>,
    mut query_ui: Query<&mut Text, With<ManaCounter>>,
) {
    let mut text = query_ui.single_mut();
    let mana = query_player.single();
    let percent = (100 * mana.0 as i32) / 100;
    let percent = if percent < 0 { 0 } else { percent };

    text.sections[0].value = percent.to_string();
}

fn update_life_counter(
    query_player: Query<&Life, With<Player>>,
    mut query_ui: Query<&mut Text, With<LifeCounter>>,
) {
    let mut text = query_ui.single_mut();
    let life = query_player.single();
    text.sections[0].value = life.0.to_string();
}

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(Playing).with_system(update_mana_counter))
            .add_system_set(SystemSet::on_update(Playing).with_system(update_life_counter))
            .add_system_set(SystemSet::on_enter(Playing).with_system(generate_ui));
    }
}
