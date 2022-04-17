use crate::prelude::*;
use bevy::utils::HashMap;
use leafwing_input_manager::user_input::UserInput;
pub use leafwing_input_manager::{prelude::*, user_input::InputButton};
use petitset::PetitSet;
use InputButton::*;
use UserInput::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Escape,
}

#[derive(Component)]
pub struct ActionManager;

pub struct KeyBindings {
    map: HashMap<Action, (UserInput, UserInput)>,
}
impl Default for KeyBindings {
    fn default() -> Self {
        let mut null: PetitSet<InputButton, 8> = PetitSet::new();
        null.insert(Keyboard(KeyCode::RAlt));
        null.insert(Keyboard(KeyCode::LAlt));
        null.insert(Keyboard(KeyCode::Colon));
        let null = Chord(null);

        let mut map: HashMap<Action, (UserInput, UserInput)> = HashMap::new();

        map.insert(
            MoveUp,
            (Single(Keyboard(KeyCode::W)), Single(Keyboard(KeyCode::Up))),
        );
        map.insert(
            MoveDown,
            (
                Single(Keyboard(KeyCode::S)),
                Single(Keyboard(KeyCode::Down)),
            ),
        );
        map.insert(
            MoveLeft,
            (
                Single(Keyboard(KeyCode::A)),
                Single(Keyboard(KeyCode::Left)),
            ),
        );
        map.insert(
            MoveRight,
            (
                Single(Keyboard(KeyCode::D)),
                Single(Keyboard(KeyCode::Right)),
            ),
        );
        map.insert(Escape, (Single(Keyboard(KeyCode::Escape)), null));

        Self { map }
    }
}
impl KeyBindings {
    fn default_input_map(&self) -> InputMap<Action> {
        let mut input_map = InputMap::default();

        for mapping in self.map.clone().into_iter() {
            input_map.insert(mapping.0, mapping.1 .0);
            input_map.insert(mapping.0, mapping.1 .1);
        }

        input_map
    }
}

fn input_manager_setup(mut commands: Commands, key_bindings: Res<KeyBindings>) {
    commands
        .spawn()
        .insert(Name::new("ActionManager"))
        .insert(ActionManager)
        .insert_bundle(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: key_bindings.default_input_map(),
        });
}

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KeyBindings>()
            .add_plugin(InputManagerPlugin::<Action>::default())
            .add_system_set(SystemSet::on_enter(AssetLoad).with_system(input_manager_setup));
    }
}
