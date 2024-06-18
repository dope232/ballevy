use bevy::prelude::*;





use crate::game::resources::Score; 
use crate::game::components::Enemy;
use crate::hud::components::EnemyText; 
use crate::hud::components::ScoreText;
use crate::AppState; 
use crate::events::GameOver;


pub fn update_score_text(mut text_query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if score.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", score.value.to_string());
        }
    }
}


pub fn update_enemy_text(
    mut text_query: Query<&mut Text, With<EnemyText>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    let count = enemy_query.iter().count();
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("{}", count.to_string());
    }
}