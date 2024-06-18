pub mod components;
mod systems;

use systems::interactions::*;
use systems::layout::*;
// use systems::updates::*;

use crate::AppState;
use bevy::prelude::*;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_system(spawn_game_over.in_schedule(OnEnter(AppState::GameOver)))
            .add_systems(
                (
                    interact_with_restart_button,
                    interact_with_quit_button,
                    
                )
                    .in_set(OnUpdate(AppState::GameOver)),
            )
            // // OnExit State Systems
            .add_system(despawn_game_over.in_schedule(OnExit(AppState::GameOver)));
    }
}