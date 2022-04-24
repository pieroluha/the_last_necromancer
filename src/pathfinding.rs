use crate::prelude::*;
use pathfinding::prelude::astar;
use std::collections::VecDeque;

const CELL_RADIUS: f32 = 8.0;
pub const CELL_SIZE: f32 = CELL_RADIUS * 2.0;

fn map_index(x: i32, y: i32) -> usize {
    ((y * ARENA_SIZE as i32) + x) as usize
}

struct Map {
    grid: Vec<u8>,
}
impl Map {
    fn in_bounds(&self, x: i32, y: i32) -> bool {
        if x >= 0 && x < ARENA_SIZE as i32 && y >= 0 && y < ARENA_SIZE as i32 {
            if self.can_enter(x, y) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn can_enter(&self, x: i32, y: i32) -> bool {
        if self.grid[map_index(x, y)] == 1 {
            true
        } else {
            false
        }
    }
}
impl Default for Map {
    fn default() -> Self {
        Self {
            grid: vec![1; ARENA_GRID_SIZE],
        }
    }
}

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (-1, 1),
    (1, -1),
    (-1, -1),
];

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self(x as i32, y as i32)
    }

    fn distance(&self, other: &Pos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }

    fn successors(&self, map: &Map) -> Vec<(Pos, u32)> {
        let &Pos(x, y) = self;

        let mut successors = Vec::new();
        for dir in DIRECTIONS {
            let _x = x + dir.0;
            let _y = y + dir.1;
            if map.in_bounds(_x, _y) {
                successors.push(Pos(_x, _y))
            } else {
                continue;
            }
        }

        successors.into_iter().map(|p| (p, 1)).collect()
    }
}

#[derive(Component)]
pub struct SelectedUnit {
    path: VecDeque<Vec2>,
    goal: Vec2,
    finish: bool,
}

impl Default for SelectedUnit {
    fn default() -> Self {
        Self {
            path: VecDeque::new(),
            goal: Vec2::ZERO,
            finish: false,
        }
    }
}

impl SelectedUnit {
    fn set_result(&mut self, result: (Vec<Pos>, u32)) {
        let mut path = VecDeque::new();

        for pos in result.0.iter() {
            path.push_back(Vec2::new((pos.0 << 4) as f32, (pos.1 << 4) as f32));
        }

        //println!("First Path {:#?}", path[0]);
        //println!("Path List {:#?}", path);

        self.path = path;
        self.finish = false;
    }

    fn set_goal(&mut self, goal: &Pos) {
        self.goal = Vec2::new((goal.0 << 4) as f32, (goal.1 << 4) as f32);
    }
}

fn assign_goal(
    map: Res<Map>,
    query_action: Query<&ActionState<Action>, With<ActionManager>>,
    cursor_position: Res<CursorPosition>,
    mut query_selected_unit: Query<(&mut SelectedUnit, &Transform)>,
) {
    let action = query_action.single();
    if !action.just_pressed(RightClick) {
        return;
    }
    let goal = vec_pos_to_grid(&cursor_position.offset_pos_integer());
    println!(
        "Goal: {}, GRID: {}",
        cursor_position.offset_pos_integer(),
        goal
    );
    if !map.in_bounds(goal.x, goal.y) {
        return;
    }
    let goal = Pos(goal.x, goal.y);
    for (mut selected_unit, transform) in query_selected_unit.iter_mut() {
        selected_unit.set_goal(&goal);
        let start = world_pos_to_grid(&transform);
        let start = Pos(start.x, start.y);

        let result = if let Some(r) = astar(
            &start,
            |p| p.successors(&map),
            |p| p.distance(&goal) / 3,
            |p| *p == goal,
        ) {
            r
        } else {
            continue;
        };

        selected_unit.set_result(result);
    }
}

const MINION_SPEED: f32 = 200.0;
const TARGET_DISTANCE: f32 = 8.0;
fn move_selected_units(
    time: Res<Time>,
    mut query_selected_unit: Query<(&mut SelectedUnit, &mut Transform)>,
) {
    for (mut selected_unit, mut transform) in query_selected_unit.iter_mut() {
        if selected_unit.path.len() == 0 {
            continue;
        }

        let target_path = selected_unit.path[0];

        if transform.translation.truncate().distance(target_path) < TARGET_DISTANCE {
            match selected_unit.path.pop_front() {
                Some(_) => (),
                None => continue,
            }
        };

        let target = look_at(&transform.translation, &target_path.extend(0.0));
        transform.translation += target * time.delta_seconds() * MINION_SPEED;

        //if get_world_pos(&transform) != selected_unit.path[0] {
        //} else {
        //}
    }
}

pub struct PathfindingPlugin;
impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Map>()
            .add_system_set(SystemSet::on_update(Playing).with_system(assign_goal))
            .add_system_set(SystemSet::on_update(Playing).with_system(move_selected_units));
    }
}
