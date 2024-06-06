use bevy::prelude::*;

#[derive(States, Debug, Default, Eq, PartialEq, Hash, Clone, Copy)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
    GameOver,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Update, game_state_input_events);
    }
}

fn game_state_input_events(
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::InGame => {
                next_state.set(GameState::Paused);
            }
            GameState::Paused => {
                next_state.set(GameState::InGame);
            }
            _ => (),
        }
    }
}

fn game_over_screen(mut event_reader: EventReader<GameState>) {
    match event_reader.read() {
        GameState::GameOver => {
            //gameover things
            info!("GAME OVER!GAME OVER!GAME OVER!GAME OVER!GAME OVER!GAME OVER!GAME OVER!GAME OVER!GAME OVER!GAME OVER!");
        }
        _ => (),
    }
}
