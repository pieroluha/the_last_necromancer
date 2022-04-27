use crate::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct UiNode;

#[derive(Component)]
struct EndMessage;

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
        .insert(UiNode)
        .with_children(|p| {
            // Health Counter
            p.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
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
            .insert(SpellCounter)
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
    let percent: f32 = if mana.0 < 0.0 {
        0.0
    } else {
        (mana.0 / MANA * 100.0).floor()
    };

    text.sections[0].value = format!("{}%", percent);
}

fn update_life_counter(
    query_player: Query<&Life, With<Player>>,
    mut query_ui: Query<&mut Text, With<LifeCounter>>,
) {
    let mut text = query_ui.single_mut();
    let life = query_player.single();

    text.sections[0].value = life.0.to_string();
}

fn update_spell_progress_counter(
    spell_progress: Res<SpellProgress>,
    mut query_ui: Query<&mut Text, With<SpellCounter>>,
) {
    let mut text = query_ui.single_mut();
    let percent = ((spell_progress.0 / 100.0) * 100.0).floor();

    text.sections[0].value = format!("{}%", percent);
}

fn update_mage_counter(
    enemy_counter: Res<EnemyCount>,
    mut query_ui: Query<&mut Text, With<MageCounter>>,
) {
    let mut text = query_ui.single_mut();

    text.sections[0].value = enemy_counter.mages.to_string();
}

fn update_archer_counter(
    enemy_counter: Res<EnemyCount>,
    mut query_ui: Query<&mut Text, With<ArcherCounter>>,
) {
    let mut text = query_ui.single_mut();

    text.sections[0].value = enemy_counter.archers.to_string();
}

fn win_game(
    mut commands: Commands,
    font_handles: Res<FontHandles>,
    query_ui_node: Query<Entity, With<UiNode>>,
) {
    let ui_node = query_ui_node.single();
    commands.entity(ui_node).despawn_recursive();
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("WinNode"))
        .with_children(|p| {
            p.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect {
                        right: Val::Auto,
                        left: Val::Auto,
                        top: Val::Percent(10.0),
                        ..default()
                    },
                    align_self: AlignSelf::FlexStart,
                    //position_type: PositionType::Absolute,
                    //position: Rect {
                    //    top: Val::Px(5.0),
                    //    left: Val::Px(90.0),
                    //    ..default()
                    //},
                    ..default()
                },
                text: Text::with_section(
                    "YOU WON!\nDo you wanna play again?",
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
            .insert(EndMessage);

            p.spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    // center button
                    margin: Rect {
                        right: Val::Auto,
                        left: Val::Auto,
                        top: Val::Percent(5.0),
                        ..default()
                    },
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::FlexEnd,
                    ..default()
                },
                color: NORMAL_BUTTON.into(),
                ..default()
            })
            .with_children(|pi| {
                pi.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Yeah...",
                        TextStyle {
                            font: font_handles.dungeon_font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    ..default()
                });
            });
        });
}

fn lose_game(
    mut commands: Commands,
    font_handles: Res<FontHandles>,
    query_ui_node: Query<Entity, With<UiNode>>,
) {
    let ui_node = query_ui_node.single();
    commands.entity(ui_node).despawn_recursive();
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("LoseNode"))
        .with_children(|p| {
            p.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect {
                        right: Val::Auto,
                        left: Val::Auto,
                        top: Val::Percent(10.0),
                        ..default()
                    },
                    align_self: AlignSelf::FlexStart,
                    //position_type: PositionType::Absolute,
                    //position: Rect {
                    //    top: Val::Px(5.0),
                    //    left: Val::Px(90.0),
                    //    ..default()
                    //},
                    ..default()
                },
                text: Text::with_section(
                    "YOU DIED!\nDo you wanna play again?",
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
            .insert(EndMessage);

            p.spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    // center button
                    margin: Rect {
                        right: Val::Auto,
                        left: Val::Auto,
                        top: Val::Percent(5.0),
                        ..default()
                    },
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::FlexEnd,
                    ..default()
                },
                color: NORMAL_BUTTON.into(),
                ..default()
            })
            .with_children(|pi| {
                pi.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Yeah...",
                        TextStyle {
                            font: font_handles.dungeon_font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    ..default()
                });
            });
        });
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text, With<EndMessage>>,
) {
    for (interaction, mut color, _children) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                for mut text in text_query.iter_mut() {
                    text.sections[0].value = "Just restart the browser.\no===>".to_string();
                }
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                //text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(Playing).with_system(update_mana_counter))
            .add_system_set(SystemSet::on_update(Playing).with_system(update_life_counter))
            .add_system_set(SystemSet::on_update(Playing).with_system(update_mage_counter))
            .add_system_set(SystemSet::on_update(Playing).with_system(update_archer_counter))
            .add_system_set(SystemSet::on_update(Winner).with_system(button_system))
            .add_system_set(SystemSet::on_update(GameOver).with_system(button_system))
            .add_system_set(SystemSet::on_enter(Winner).with_system(win_game))
            .add_system_set(SystemSet::on_enter(GameOver).with_system(lose_game))
            .add_system_set(
                SystemSet::on_update(Playing).with_system(update_spell_progress_counter),
            )
            .add_system_set(SystemSet::on_enter(Playing).with_system(generate_ui));
    }
}
