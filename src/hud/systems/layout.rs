use bevy::prelude::*;
use bevy::utils::Duration;
use crate::hud::components::*; 


pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_hud(&mut commands, &asset_server);
}

pub fn build_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let hud_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(15.0)),
                    ..default()
                },
                ..default()
            },
            HUD {},
        ))
        .with_children(|parent| {
            // LHS
            parent
                .spawn(NodeBundle {
                    style: Style{
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Px(200.0), Val::Percent(80.0)),
                        margin: UiRect::new(Val::Px(32.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0)),
                        ..default()
                    },
                    background_color: Color::rgba(102.0 / 255.0, 102.0 / 255.0, 102.0 / 255.0, 0.5).into(),

                    ..default()
                })
                .with_children(|parent| {
                    // Star Image
                    parent.spawn(ImageBundle {
                        style: Style{
                            size: Size::new(Val::Px(48.0), Val::Px(48.0)),
                            margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
                            ..default()
                        },
                        image: asset_server.load("sprites/star.png").into(),
                        ..default()
                    });
                    // Score Text
                    parent.spawn((
                        TextBundle {
                            style: Style { ..default() },
                            text: Text {
                                sections: vec![TextSection::new(
                                    "0",
                                    TextStyle {
                                        font: asset_server.load("fonts/DisposableDroidBB_bld.ttf"),
                                        font_size: 64.0,
                                        color: Color::WHITE.into(),
                                    },
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        ScoreText {},
                    ));
                });
            // RHS
            parent
                .spawn(NodeBundle {
                    style: Style{
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Px(200.0), Val::Percent(80.0)),
                        margin: UiRect::new(Val::Px(0.0), Val::Px(32.0), Val::Px(0.0), Val::Px(0.0)),
                        ..default()
                    },
                    background_color: Color::rgba(102.0 / 255.0, 102.0 / 255.0, 102.0 / 255.0, 0.5).into(),

                    ..default()
                })
                .with_children(|parent| {
                    // Enemy Text
                    parent.spawn((
                        TextBundle {
                            style: Style { ..default() },
                            text: Text {
                                sections: vec![TextSection::new(
                                    "0",
                                    TextStyle {
                                        font: asset_server.load("fonts/DisposableDroidBB_bld.ttf"),
                                        font_size: 64.0,
                                        color: Color::WHITE.into(),
                                    },

                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        EnemyText {
                           
                        },
                    ));
                    
                    parent.spawn(ImageBundle {
                        style: Style{
                            size: Size::new(Val::Px(48.0), Val::Px(48.0)),
                            margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
                            ..default()
                        },
                        image: asset_server.load("sprites/ball_blue_small.png").into(),
                        ..default()
                    });
                });
        })
        .id();

    hud_entity
}




pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<HUD>>) {
    for entity in hud_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}