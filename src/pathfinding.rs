use crate::prelude::*;
use pathfinding::prelude::astar;
use std::collections::VecDeque;
//use std::f32::consts::PI;

//const P2: f32 = PI * 2.0;
const CELL_RADIUS: f32 = 8.0;
pub const CELL_SIZE: f32 = CELL_RADIUS * 2.0;

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

// From the pathfinding crate
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub i32, pub i32);

impl Pos {
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

        // Cost?
        successors.into_iter().map(|p| (p, 1)).collect()
    }

    fn bound_pos(pos: IVec2) -> Self {
        let arena_size = ARENA_SIZE as i32;
        let x = if pos.x < 0 {
            6
        } else if pos.x >= arena_size {
            arena_size - 6
        } else {
            pos.x
        };

        let y = if pos.y < 0 {
            6
        } else if pos.y >= arena_size {
            arena_size - 6
        } else {
            pos.y
        };

        Self(x, y)
    }
}

fn assign_goal(
    map: Res<Map>,
    query_action: Query<&ActionState<Action>, With<ActionManager>>,
    cursor_position: Res<CursorPosition>,
    selected_entities: Res<SelectedEntities>,
    mut query_selected_unit: Query<(&mut SelectedUnit, &Transform)>,
) {
    let action = query_action.single();
    if !action.just_pressed(RightClick) {
        return;
    }

    let mut goals = VecDeque::new();
    let goal = vec_pos_to_grid(&cursor_position.offset_pos_integer());

    if !map.in_bounds(goal.x, goal.y) {
        return;
    }

    let entity_count = selected_entities.0.len();
    if entity_count == 1 {
        goals.push_back(goal);
    } else {
        goals.push_back(goal);
        let mut goal_dist = 1;
        for i in 1..entity_count {
            if i % 8 == 0 {
                goal_dist += 1;
            }

            //let angle = i as f32 * (P2 / entity_count as f32);
            //rotate2d(Vec2::Y, angle);

            let dir = DIRECTIONS[i % 8];
            let dir = IVec2::new(dir.0, dir.1);
            let pos = goal + dir * goal_dist;
            goals.push_back(pos);
        }
    }

    for (mut selected_unit, transform) in query_selected_unit.iter_mut() {
        let goal = if let Some(g) = goals.pop_front() {
            g
        } else {
            continue;
        };

        let goal = if map.in_bounds(goal.x, goal.y) {
            Pos(goal.x, goal.y)
        } else {
            Pos::bound_pos(goal)
        };

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
const TARGET_DISTANCE: f32 = 4.0;
fn move_selected_units(
    time: Res<Time>,
    mut query_selected_unit: Query<(&mut SelectedUnit, &mut Transform, &mut AnimState)>,
) {
    for (mut selected_unit, mut transform, mut minion_state) in query_selected_unit.iter_mut() {
        if selected_unit.is_selected == false {
            continue;
        }

        if selected_unit.path.len() == 0 {
            if *minion_state == AnimState::Run {
                *minion_state = AnimState::Idle;
            }
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

        if *minion_state == AnimState::Idle {
            *minion_state = AnimState::Run;
        }
    }
}

pub struct PathfindingPlugin;
impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(Playing).with_system(assign_goal))
            .add_system_set(SystemSet::on_update(Playing).with_system(move_selected_units));
    }
}
