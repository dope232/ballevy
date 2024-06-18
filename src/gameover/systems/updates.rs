use bevy::prelude::*;
use crate::events::GameOver;
use crate::gameover::components::FinalScoreText;

// pub fn update_final_score_text(
//     mut game_over_event_reader: EventReader<GameOver>,
//     mut text_query: Query<&mut Text, With<FinalScoreText>>,
// ) {
//     for event in game_over_event_reader.iter() {
//         println!("Received GameOver event with score: {}", event.score); // Debug output
//         for mut text in text_query.iter_mut() {
//             println!("Updating text to: Final Score: {}", event.score); // Debug output
//             text.sections[0].value = format!("Final Score: {}", event.score);
//         }
//     }
// }
