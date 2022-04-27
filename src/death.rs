use crate::prelude::*;

fn kill_lifeless_minions(
    query_minions: Query<(Entity, &Life), With<Minion>>,
    mut commands: Commands,
    mut play_sfx: ResMut<PlaySfx>,
) {
    for (minion, life) in query_minions.iter() {
        if life.0 == 0 {
            play_sfx.death = true;
            commands.entity(minion).despawn_recursive();
        }
    }
}

fn kill_lifeless_enemies(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    mut play_sfx: ResMut<PlaySfx>,
    query_enemies: Query<(Entity, &Life, &mut Enemy)>,
) {
    for (enemy, life, enemy_type) in query_enemies.iter() {
        if life.0 == 0 {
            play_sfx.oof = true;
            commands.entity(enemy).despawn_recursive();

            match enemy_type.0 {
                EnemyType::Archer(true) | EnemyType::Archer(false) => {
                    enemy_count.archers = enemy_count.archers.saturating_sub(1);
                }
                EnemyType::Mage(true) | EnemyType::Mage(false) => {
                    enemy_count.mages = enemy_count.archers.saturating_sub(1);
                }
            }

            enemy_count.current = enemy_count.current.saturating_sub(1);
        }
    }
}

fn winner(enemy_count: Res<EnemyCount>, mut app_state: ResMut<State<GameState>>) {
    if enemy_count.archers == 0 && enemy_count.mages == 0 {
        app_state.set(Winner).unwrap();
    }
}

fn game_over(query_player: Query<&Life, With<Player>>, mut app_state: ResMut<State<GameState>>) {
    let player_life = query_player.single();

    if player_life.0 == 0 {
        app_state.set(GameOver).unwrap();
    }
}

pub struct DeathPlugin;
impl Plugin for DeathPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(Playing).with_system(kill_lifeless_minions.label("kill_minions")),
        )
        .add_system_set(SystemSet::on_update(Playing).with_system(winner))
        .add_system_set(SystemSet::on_update(Playing).with_system(game_over))
        .add_system_set(SystemSet::on_update(Playing).with_system(kill_lifeless_enemies));
    }
}
