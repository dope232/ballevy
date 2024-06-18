use bevy::prelude::*;

use crate:: mainmenu::components::{MainMenu, PlayButton, QuitButton}; 


pub fn spawn_main_menu(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {


    build_main_menu(&mut commands, &asset_server); 


}


pub fn despawn_main_menu(
    mut commands: Commands, 
    main_menu_query: Query<Entity, With<MainMenu>>
){

    if let Ok(main_menu_entity) = main_menu_query.get_single(){
        commands.entity(main_menu_entity).despawn_recursive();

    }

}

pub fn build_main_menu(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>

)-> Entity{
    let background_image_handle = asset_server.load("sprites/image.png");
    let main_menu_entity = commands.spawn(
        (NodeBundle{
            style: Style{
                flex_direction: FlexDirection::Column, 
                justify_content: JustifyContent::Center, 
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)), 
                gap: Size::new(Val::Px(8.0), Val::Px(8.0)), 
                  ..default()



            }, 


            background_color: Color::NONE.into(),
           
            ..default()


        },
    MainMenu{}) )

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

            //img 1 

            // parent.spawn(
            //     (
            //         ImageBundle{
            //             style: Style{
            //                 size:Size::new(Val::Px(64.0), Val::Px(64.0)),
            //                 margin:UiRect::new(Val::Px(8.0),Val::Px(8.0),Val::Px(8.0),Val::Px(8.0)),
            //                 ..default()




            //             }, 
            //             image: asset_server.load(""),
            //             ..default()
            //         }

            //     )
            // )

            //title 

            parent.spawn(
                TextBundle{
                    text : Text{
                        sections: (
                            vec![
                                TextSection::new(
                                    "BALLEVY", 
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




                    background_color: Color::CYAN.into(), 
                    ..default()

                }, 


                PlayButton {}, 


            ))
            .with_children(|parent|{

                parent.spawn(
                    (
                    TextBundle {
                        text: Text { 
                            sections: (
                                vec![
                                    TextSection::new(
                                        "Play", 
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

                    PlayButton{}, 


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




                    background_color: Color::CYAN.into(), 
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

        parent.spawn(
            TextBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart, 
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                text: Text {
                    sections: vec![
                        TextSection::new(
                            "WASD to move; Space to Pause",
                            TextStyle {
                                font: asset_server.load("fonts/DisposableDroidBB_bld.ttf"),
                                font_size: 15.0,
                                color: Color::WHITE.into(),
                            },
                        ),
                        TextSection::new(
                            "- Enemies spawn periodically; Try to get as many stars as possible!",
                            TextStyle {
                                font: asset_server.load("fonts/DisposableDroidBB_bld.ttf"),
                                font_size: 15.0,
                                color: Color::WHITE.into(),
                            },
                        ),
                    ],
                    alignment: (
                        TextAlignment::Center

                    ),
                    
                    ..default()
                },

                
                
                ..default()
            },
        );
        

        


    })

    .id(); 



    main_menu_entity

}