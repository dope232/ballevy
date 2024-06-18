mod gameover;
mod mainmenu;
mod game;
mod hud;





use bevy::prelude::*;

use gameover::GameOverMenuPlugin;
use mainmenu::MainMenuPlugin;
use hud::HUDPlugin;  



use game::*; 





fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugins)
        .add_plugin(HUDPlugin)
        .add_plugin(GameOverMenuPlugin)
        .add_system(trans_to_game_state)
        .add_system(trans_to_main_menu_state)




        .run();
}

#[derive(States,Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,   
    GameOver,
}

pub fn trans_to_game_state(
    mut commands: Commands, 
    keyboard_input: Res<Input<KeyCode>>, 
    app_state: Res<State<AppState>>, 
){

    if keyboard_input.pressed(KeyCode::G){
        if app_state.0!= AppState::Game {
            commands.insert_resource(NextState(Some(AppState::Game))); 
            
            println!("In the game"); 

        }
    }

}

pub fn trans_to_main_menu_state(
    mut commands: Commands, 
    keyboard_input: Res<Input<KeyCode>>, 
    app_state: Res<State<AppState>>, 
){

    if keyboard_input.pressed(KeyCode::Tab){
        if app_state.0!= AppState::MainMenu {
            commands.insert_resource(NextState(Some(AppState::MainMenu))); 
            commands.insert_resource(NextState(Some(GameState::Paused)));
            println!("In the main menu"); 

        }
    }


}


