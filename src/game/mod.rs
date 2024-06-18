pub mod components;
pub mod events;
pub mod resources;
pub mod systems;



use events::*;
use resources::*;
use systems::*;



use bevy::prelude::*;
use crate::AppState; 

pub struct GamePlugins; 

impl Plugin for GamePlugins {
    fn build(&self, app: &mut App){
        app.add_event::<GameOver>()
        .add_state::<GameState>()
        .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
        
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()

        .add_startup_system(spawn_camera)

        .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
        .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
        .add_system(spawn_star.in_schedule(OnEnter(AppState::Game)))

        .add_systems(
            (
                player_movement,
                confine_player_movement,
                player_hitting_star,


            )
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(GameState::Running))

        )
        
        
        .add_systems(
            (
                enemy_movement,
                update_enemy_direction,
                confine_enemy_movement,
                enemy_hit_player,
                tick_enemy_spawn_timer,
                spawn_enemy_over_time,




            )
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(GameState::Running))
        )
        
        .add_system(insert_score
        .in_schedule(OnEnter(AppState::Game))
    )
        

        
        .add_system(update_score
            
            .run_if(in_state(AppState::Game))
        )

        .add_systems(
            (
                tick_star_spawn_timer,
                spawn_stars_over_time,
            )
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(GameState::Running))


        )
        .add_system(despawn_players.in_schedule(OnExit(AppState::Game)))
        .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)))
        .add_system(despawn_stars.in_schedule(OnExit(AppState::Game)))
        .add_system(delete_score.in_schedule(OnExit(AppState::Game)))

        
        .add_system(exit_game)
        .add_system(handle_game_over)
        .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
        .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));

        
        
            
        

    }

}



#[derive(States,Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    
    #[default]
    Running,
    Paused
}