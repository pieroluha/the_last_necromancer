use crate::prelude::*;

#[derive(Component)]
struct SelectionBox;

#[derive(Default)]
pub struct SelectedEntities(pub Vec<Entity>);

const TRANS_GREEN: Color = Color::rgba(104.0 / 255.0, 110.0 / 255.0, 70.0 / 255.0, 50.0 / 255.0);

fn initialize_selection_box(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: TRANS_GREEN,
                ..default()
            },
            ..default()
        })
        .insert(Name::new("SelectionBox"))
        .insert(RectAABB::default())
        .insert(SelectionBox);
}

struct SelectEvent(RectAABB);

fn update_selection_box(
    cursor_position: Res<CursorPosition>,
    query_action: Query<&ActionState<Action>, With<ActionManager>>,
    mut query_selection_box: Query<&mut Transform, With<SelectionBox>>,
    mut select_event: EventWriter<SelectEvent>,
    mut selected_entities: ResMut<SelectedEntities>,
    mut origin: Local<Vec2>,
    mut pos: Local<Vec2>,
    mut size: Local<Vec2>,
    mut jimbo: Local<bool>,
    mut commands: Commands,
) {
    let mut action = query_action.single();
    let mut transform = query_selection_box.single_mut();

    if action.released(LeftClick) {
        transform.scale = Vec3::ZERO;
        if *jimbo == false {
            return;
        }
        select_event.send(SelectEvent(RectAABB {
            pos: *pos,
            size: *size,
        }));
        *jimbo = false;
    }

    if action.just_pressed(LeftClick) {
        *origin = cursor_position.pos;
        for entity in selected_entities.0.iter() {
            commands.entity(*entity).remove::<SelectedUnit>();
        }
        selected_entities.0.clear();
        *jimbo = true;
    }

    let end = cursor_position.pos;

    let lower_left = Vec2::new(origin.x.min(end.x), origin.y.min(end.y));
    let upper_right = Vec2::new(origin.x.max(end.x), origin.y.max(end.y));

    *size = upper_right - lower_left;
    *pos = lower_left + (*size * 0.5);

    transform.scale = size.extend(0.0);
    transform.translation = pos.extend(2.0);
}

fn rect_intersection(
    query_minion: Query<(&RectAABB, Entity), (With<Minion>, Without<SelectionBox>)>,
    mut query_selection_box: Query<&mut RectAABB, (With<SelectionBox>, Without<Minion>)>,
    mut selected_entities: ResMut<SelectedEntities>,
    mut read_select_event: EventReader<SelectEvent>,
    mut commands: Commands,
) {
    let mut rect_box = query_selection_box.single_mut();

    for select_box in read_select_event.iter() {
        for (rect_minion, minion) in query_minion.iter() {
            if select_box.0.collision_check(rect_minion) {
                selected_entities.0.push(minion);
                commands.entity(minion).insert(SelectedUnit::default());
            }
        }
    }
}

pub struct SelectionBoxPlugin;
impl Plugin for SelectionBoxPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedEntities>()
            .add_event::<SelectEvent>()
            .add_system_set(SystemSet::on_enter(Playing).with_system(initialize_selection_box))
            .add_system_set(SystemSet::on_update(Playing).with_system(rect_intersection))
            .add_system_set(SystemSet::on_update(Playing).with_system(update_selection_box));
    }
}
