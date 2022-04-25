use crate::prelude::*;

fn kill_lifeless_minions(
    mut commands: Commands,
    query_minions: Query<(Entity, &Life), With<Minion>>,
) {
    for (minion, life) in query_minions.iter() {
        if life.0 == 0 {
            print!("Dead Minion");
            commands.entity(minion).despawn_recursive();
        }
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
        );
    }
}
