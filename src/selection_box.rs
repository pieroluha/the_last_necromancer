use crate::prelude::*;
use std::collections::VecDeque;

#[derive(Component)]
struct SelectionBox;

#[derive(Default)]
pub struct SelectedEntities(pub Vec<Entity>);

struct SelectEvent(RectAABB);

#[derive(Component)]
pub struct SelectedUnit {
    pub path: VecDeque<Vec2>,
    pub goal: Vec2,
    pub is_selected: bool,
}

impl Default for SelectedUnit {
    fn default() -> Self {
        Self {
            path: VecDeque::new(),
            goal: Vec2::ZERO,
            is_selected: false,
        }
    }
}

impl SelectedUnit {
    pub fn set_result(&mut self, result: (Vec<Pos>, u32)) {
        let mut path = VecDeque::new();

        for pos in result.0.iter() {
            path.push_back(Vec2::new((pos.0 << 4) as f32, (pos.1 << 4) as f32));
        }

        //println!("First Path {:#?}", path[0]);
        //println!("Path List {:#?}", path);

        self.path = path;
    }

    pub fn set_goal(&mut self, goal: &Pos) {
        self.goal = Vec2::new((goal.0 << 4) as f32, (goal.1 << 4) as f32);
    }

    pub fn clear(&mut self) {
        self.path = VecDeque::new();
        self.goal = Vec2::ZERO;
        self.is_selected = false;
    }
}

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
        .insert(SelectionBox);
}

fn update_selection_box(
    cursor_position: Res<CursorPosition>,
    query_action: Query<&ActionState<Action>, With<ActionManager>>,
    mut query_minions: Query<&mut SelectedUnit>,
    mut query_selection_box: Query<&mut Transform, With<SelectionBox>>,
    mut select_event: EventWriter<SelectEvent>,
    mut selected_entities: ResMut<SelectedEntities>,
    mut origin: Local<Vec2>,
    mut pos: Local<Vec2>,
    mut size: Local<Vec2>,
    mut jimbo: Local<bool>,
) {
    let action = query_action.single();
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
        selected_entities.0.clear();
        for mut minion in query_minions.iter_mut() {
            minion.clear();
        }
        *jimbo = true;
    }

    let end = cursor_position.pos;

    let lower_left = Vec2::new(origin.x.min(end.x), origin.y.min(end.y));
    let upper_right = Vec2::new(origin.x.max(end.x), origin.y.max(end.y));

    *size = upper_right - lower_left;
    *pos = lower_left + (*size * 0.5);

    transform.scale = size.extend(0.0);
    transform.translation = pos.extend(5.0);
}

fn rect_intersection(
    mut query_minion: Query<(&RectAABB, Entity, &mut SelectedUnit), With<Minion>>,
    mut selected_entities: ResMut<SelectedEntities>,
    mut read_select_event: EventReader<SelectEvent>,
) {
    for select_box in read_select_event.iter() {
        for (rect_minion, minion, mut selected) in query_minion.iter_mut() {
            if select_box.0.collision_check(rect_minion) {
                selected.is_selected = true;
                selected_entities.0.push(minion);
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
            .add_system_set(
                SystemSet::on_update(Playing).with_system(
                    rect_intersection
                        .label("rect_intersection")
                        .after("update_selection"),
                ),
            )
            .add_system_set(
                SystemSet::on_update(Playing)
                    .with_system(update_selection_box.label("update_selection")),
            );
    }
}
