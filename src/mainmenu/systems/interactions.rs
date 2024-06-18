use bevy::{app::AppExit, prelude::*};
use crate::AppState; 
use crate::mainmenu::components::QuitButton; 
use crate::mainmenu::components::PlayButton; 


pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {


    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction{
            Interaction::Clicked => {
                *background_color = Color::DARK_GRAY.into(); 
                app_state_next_state.set(AppState::Game); 


            
        }
        Interaction::Hovered => {
            *background_color = Color::hex("#d3d3d3").unwrap().into(); 
        }
        Interaction::None =>{

            *background_color = Color::GRAY.into(); 

        }
    }



}
}

pub fn interact_with_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = Color::DARK_GRAY.into(); 
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                *background_color = Color::RED.into(); 
            }
            Interaction::None => {
                *background_color = Color::GRAY.into(); 
            }
        }
    }
}