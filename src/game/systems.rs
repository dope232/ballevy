use bevy::prelude::*;

use bevy::window::PrimaryWindow;
use rand::prelude::*;
use bevy::app::AppExit; 

use crate::components::*; 
use crate::resources::*; 
use crate::events::*;
use crate::GameState;
use crate::AppState;


use crate::gameover::components::FinalScoreText;



pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; 
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0; 
pub const NO_OF_STARS: usize = 14; 
pub const STAR_SIZE: f32 = 30.0; 


pub fn pause_simulation(
    mut game_state_next_state: ResMut<NextState<GameState>>, 

){
    game_state_next_state.set(GameState::Paused); 

}

pub fn resume_simulation(
    mut game_state_next_state: ResMut<NextState<GameState>>, 

){
    game_state_next_state.set(GameState::Running); 

}

pub fn toggle_simulation(
    mut commands: Commands, 
    keyboard_input: Res<Input<KeyCode>>, 
    simulation_state: Res<State<GameState>>

){
    if keyboard_input.just_pressed(KeyCode::Space){
        if simulation_state.0 == GameState::Running {
            commands.insert_resource(NextState(Some(GameState::Paused))); 
            println!("Game paused"); 
        }
        if simulation_state.0 == GameState::Paused {
            commands.insert_resource(NextState(Some(GameState::Running))); 
            println!("Game resumed"); 
        }
    }

}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0, 
                window.height() / 2.0,
                0.0),
            texture: asset_server.load("sprites/ball_red_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            window.width() / 2.0, 
            window.height() / 2.0, 
            0.0),
        ..default()
    });
}

pub fn despawn_players(
    mut commands: Commands, 
    player_query: Query<Entity, With<Player>>, 



){

    if let Ok(player_entity) = player_query.get_single(){
        commands.entity(player_entity).despawn(); 
    }
    

}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    random_x, 
                    random_y, 
                    0.0),
                texture: asset_server.load("sprites/ball_blue_small.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn despawn_enemies(
    mut commands: Commands, 
    enemy_query: Query<Entity, With<Enemy>>, 



){
    for enemy_entity in enemy_query.iter(){
        commands.entity(enemy_entity).despawn(); 
    }
}


pub fn spawn_star(
    mut commands: Commands, 
    window_query: Query<&Window, With<PrimaryWindow>>, 
    asset_server: Res<AssetServer>,
){
    let window = window_query.get_single().unwrap();
    for _ in 0..NO_OF_STARS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {
                
            },
        ));


    }


}


pub fn despawn_stars(
    mut commands: Commands, 
    star_query: Query<Entity, With<Star>>, 



){
    for star_entity in star_query.iter(){
        commands.entity(star_entity).despawn(); 
    }
}



pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left)  {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right)  {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down){
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0; // 32.0
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {

    for (mut transform, enemy) in enemy_query.iter_mut() {

        let direction = Vec3::new(
            enemy.direction.x,
            enemy.direction.y,
         0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0; // 32.0
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;

        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        // Play SFX
        if direction_changed {
            // Play Sound Effect
            let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");
            // Randomly play one of the two sound effects.
            let sound_effect = if random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };
            audio.play(sound_effect);
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        // Bound the enemy x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        // Bound the enemy y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    mut game_over_event_writer: EventWriter<GameOver>, 
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    score: Res<Score>, 
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("Enemy hit player! Game Over!");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver {score: score.value}); 
            }
        }
    }
}




pub fn player_hitting_star(
    mut commands: Commands, 
     player_query: Query<&Transform, With<Player>>, 
     star_query: Query<(Entity, &Transform),With<Star>>, 
     asset_server: Res<AssetServer>, 
     audio: Res<Audio>, 
     mut score: ResMut<Score>,

    
    
) {

    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
            .translation
            .distance(star_transform.translation); 
            
            if distance < PLAYER_SIZE/ 2.0 + STAR_SIZE / 2.0 {
                println!("Player hit star"); 
                score.value +=1; 
                let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
                audio.play(sound_effect);
                commands.entity(star_entity).despawn(); 


            }
        }

    

}
}

pub fn update_score(score: ResMut<Score>) {
    if score.is_changed() {
        println!("Score {}", score.value.to_string()); 

    }
    
}


pub fn insert_score(mut commands: Commands){
    commands.insert_resource(Score::default())
}

pub fn delete_score(mut commands: Commands){
    commands.remove_resource::<Score>(); 

}




pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}


pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}



pub fn spawn_enemy_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_blue_small.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(
                    random::<f32>(),
                    random::<f32>()
                    
                ).normalize(), 
            },

        ));
    }
}


pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>, 
    mut app_exit_event_writer: EventWriter<AppExit>, 
){
    if keyboard_input.pressed(KeyCode::Escape){
        app_exit_event_writer.send(AppExit); 
    }

}

pub fn handle_game_over(
    mut commands: Commands,
    mut game_over_event_reader: EventReader<GameOver>,
    mut text_query: Query<&mut Text, With<FinalScoreText>>,
) {
    for event in game_over_event_reader.iter() {
        println!("Your final score is {}", event.score.to_string());
        
        // Update the text in the UI
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("Final Score: {}", event.score);
        }

        // Transition to GameOver state
        commands.insert_resource(NextState(Some(AppState::GameOver)));
    }
}

