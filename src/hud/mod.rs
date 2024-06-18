mod components;
mod systems;

use bevy::prelude::*;
use systems::layout::*;
use systems::updates::*;

use crate::AppState;
pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(spawn_hud.in_schedule(OnEnter(AppState::Game)))
        // Systems
        .add_systems((update_score_text, update_enemy_text).in_set(OnUpdate(AppState::Game)))
        // OnExit Systems
        .add_system(despawn_hud.in_schedule(OnExit(AppState::Game)));
            
    }
}
    