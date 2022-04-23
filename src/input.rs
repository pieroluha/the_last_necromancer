use crate::prelude::*;
use leafwing_input_manager::user_input::UserInput;
pub use leafwing_input_manager::{prelude::*, user_input::InputButton};
use InputButton::*;
use UserInput::*;

// If regenerating mana turn off shield
// If not, turn on shield
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    RightClick,
    LeftClick,
    SelectAll,
    ManaToggle,
    RemoveSelect,
    Pause,
}

impl Action {
    fn default_input_map() -> InputMap<Action> {
        let mut input_map = InputMap::default();

        input_map.insert(RightClick, Single(Mouse(MouseButton::Right)));
        input_map.insert(LeftClick, Single(Mouse(MouseButton::Left)));
        input_map.insert(SelectAll, Single(Keyboard(KeyCode::A)));
        input_map.insert(ManaToggle, Single(Keyboard(KeyCode::Space)));
        input_map.insert(RemoveSelect, Single(Keyboard(KeyCode::Q)));
        input_map.insert(Pause, Single(Keyboard(KeyCode::Escape)));

        input_map
    }
}

#[derive(Component)]
pub struct ActionManager;

fn input_manager_setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(Name::new("ActionManager"))
        .insert(ActionManager)
        .insert_bundle(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: Action::default_input_map(),
        });
}

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Action>::default())
            .add_system_set(SystemSet::on_enter(AssetLoad).with_system(input_manager_setup));
    }
}
