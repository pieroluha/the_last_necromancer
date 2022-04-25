use crate::prelude::*;

#[derive(Component)]
struct HealthCounter;
#[derive(Component)]
struct ManaCounter;

#[derive(Component)]
struct MageCounter;

#[derive(Component)]
struct ArcherCounter;

// Thank you inspector_egui for getting me there!
// I have little hours left and i've no time to understand this mess
fn generate_ui(
    mut commands: Commands,
    font_handles: Res<FontHandles>,
    image_handles: Res<ImageHandles>,
) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Relative,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(80.0),
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
        .insert(Name::new("PlayerUi"))
        .insert(HealthCounter)
        .with_children(|p| {
            // bevy logo (image)
            p.spawn_bundle(ImageBundle {
                style: Style {
                    align_self: AlignSelf::FlexStart,
                    position: Rect {
                        right: Val::Px(70.0),
                        ..default()
                    },
                    size: Size::new(Val::Px(64.0), Val::Auto),
                    ..default()
                },
                image: image_handles.full_heart.clone().into(),
                ..default()
            });

            p.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Relative,
                    position: Rect {
                        right: Val::Px(70.0),
                        top: Val::Px(70.0),
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

            p.spawn_bundle(ImageBundle {
                style: Style {
                    align_self: AlignSelf::FlexStart,
                    position: Rect {
                        right: Val::Px(233.0),
                        top: Val::Px(70.0),
                        ..default()
                    },
                    size: Size::new(Val::Px(64.0), Val::Auto),
                    ..default()
                },
                image: image_handles.full_mana.clone().into(),
                ..default()
            });

            p.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        right: Val::Px(438.0),
                        top: Val::Px(160.0),
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

            p.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Relative,
                    flex_direction: FlexDirection::Column,
                    position: Rect {
                        right: Val::Px(233.0),
                        top: Val::Px(217.0),
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
                    position_type: PositionType::Relative,
                    flex_direction: FlexDirection::Column,
                    position: Rect {
                        right: Val::Px(348.0),
                        top: Val::Px(153.0),
                        ..default()
                    },
                    size: Size::new(Val::Px(48.0), Val::Auto),
                    ..default()
                },
                image: image_handles.mage_icon.clone().into(),
                ..default()
            })
            .insert(Name::new("MageIcon"));

            p.spawn_bundle(ImageBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Relative,
                    flex_direction: FlexDirection::Column,
                    position: Rect {
                        right: Val::Px(397.0),
                        top: Val::Px(210.0),
                        ..default()
                    },
                    size: Size::new(Val::Px(48.0), Val::Auto),
                    ..default()
                },
                image: image_handles.archer_icon.clone().into(),
                ..default()
            })
            .insert(Name::new("ArcherIcon"));

            p.spawn_bundle(ImageBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Relative,
                    flex_direction: FlexDirection::Column,
                    position: Rect {
                        right: Val::Px(447.0),
                        top: Val::Px(300.0),
                        ..default()
                    },
                    size: Size::new(Val::Px(48.0), Val::Auto),
                    ..default()
                },
                image: image_handles.spell_icon.clone().into(),
                ..default()
            })
            .insert(Name::new("SpellIcon"));

            p.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Relative,
                    flex_direction: FlexDirection::Column,
                    position: Rect {
                        right: Val::Px(437.0),
                        top: Val::Px(302.0),
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
            .insert(Name::new("SpellCounter"));
        });
}

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(Playing).with_system(generate_ui));
    }
}
