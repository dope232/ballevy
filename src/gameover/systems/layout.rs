
use bevy::prelude::*;

use crate::gameover::components::{FinalScoreText, GameOverMenu, QuitButton, RestartButton};


 





pub fn spawn_game_over(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,

    

) {


    build_game_over(&mut commands, &asset_server); 


}


pub fn despawn_game_over(
    mut commands: Commands, 
    game_over_query: Query<Entity, With<GameOverMenu>>,
     
){

    if let Ok(game_over_entity) = game_over_query.get_single(){
        commands.entity(game_over_entity).despawn_recursive();

    }

}



pub fn build_game_over(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>,
    
    
    
    

)->Entity{
    
    let background_image_handle = asset_server.load("sprites/image.png");
    let game_over_entity = commands.spawn(
        (NodeBundle{
            style: Style{
                flex_direction: FlexDirection::Column, 
                justify_content: JustifyContent::Center, 
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)), 
                gap: Size::new(Val::Px(8.0), Val::Px(8.0)), 
                  ..default()



            }, 


            background_color: Color::BLACK.into(), 
            ..default()


        },
    GameOverMenu{}) )

    .with_children(|parent| {


        parent.spawn(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            image: UiImage {
                texture: background_image_handle.clone(),
                flip_x: false,
                flip_y: false,
            },
            ..default()
        });

        //title


        parent.spawn(
            

                NodeBundle{
                    style: Style{
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center, 
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Px(300.0), Val::Px(120.0)),
                        ..default()

                    }, 
                    ..default()
                }
        ).with_children(|parent| {

            

            parent.spawn(
                TextBundle{
                    text : Text{
                        sections: (
                            vec![
                                TextSection::new(
                                    "F :( ", 
                                    TextStyle {
                                        font: asset_server.load("fonts/DisposableDroidBB_bld.ttf"), 
                                        font_size: 85.0, 
                                        color: Color::WHITE.into(), 


                                        

                                    }
                                )

                            ]
                            
                        ),
                        alignment: (
                            TextAlignment::Center

                        ),
                        ..default()
                    }, 
                    ..default()
                }
            );




        }); 
            // final score

        parent.spawn(
            (
                TextBundle{
                    text: Text { 
                        sections: (
                            vec![TextSection::new(
                                "",
                                TextStyle { 
                                    font: asset_server.load("fonts/DisposableDroidBB_bld.ttf"),
                                    font_size: 35.0,
                                    color: Color::WHITE.into(),
                                 }
                            )]
                        ),
                         alignment: 
                            TextAlignment::Center,
                         ..default() }, 
                         ..default()

                }, 
                FinalScoreText{},
            )
        );


        //play
        parent.spawn(
            (
                ButtonBundle {

                    style: Style{
                        align_items: AlignItems::Center, 
                        justify_content: JustifyContent::Center, 



                        size: Size::new(Val::Px(200.0), Val::Px(80.0)),


                        ..default()

                    }, 




                    background_color: Color::GRAY.into(), 
                    ..default()

                }, 


                RestartButton {}, 


            ))
            .with_children(|parent|{

                parent.spawn(
                    (
                    TextBundle {
                        text: Text { 
                            sections: (
                                vec![
                                    TextSection::new(
                                        "Restart", 
                                        TextStyle {
                                            font: asset_server.load("fonts/DisposableDroidBB_bld.ttf"), 
                                            font_size: 55.0, 
                                            color: Color::WHITE.into(), 


                                            

                                        }
                                    )

                                ]
                                
                            ),
                            alignment: (
                                TextAlignment::Center

                            ),
                            ..default()

                         },

                        ..default()


                    }, 

                    RestartButton{}, 


                )
            );


            }
        );

        //quit 

        parent.spawn(
            (
                ButtonBundle {

                    style: Style{
                        align_items: AlignItems::Center, 
                        justify_content: JustifyContent::Center, 



                        size: Size::new(Val::Px(200.0), Val::Px(80.0)),


                        ..default()

                    }, 




                    background_color: Color::GRAY.into(), 
                    ..default()

                }, 


                QuitButton {}, 


            ))
            .with_children(|parent|{

                parent.spawn(
                    (
                    TextBundle {
                        text: Text { 
                            sections: (
                                vec![
                                    TextSection::new(
                                        "Quit", 
                                        TextStyle {
                                            font: asset_server.load("fonts/DisposableDroidBB_bld.ttf"), 
                                            font_size: 55.0, 
                                            color: Color::WHITE.into(), 


                                            

                                        }
                                    )

                                ]
                                
                            ),
                            alignment: (
                                TextAlignment::Center

                            ),
                            ..default()

                         },

                        ..default()


                    }, 

                    QuitButton{}, 


                )
            );


            }
        );

       

        


    })

    .id(); 



    game_over_entity

}