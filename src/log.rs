/*
 * A log of asteroids the player missed - display question and answer
 * */

use crate::flashcards::Flashcard;

pub const MESSAGE_DURATION: f32 = 10.0; //In seconds

pub struct LogItem {
    timer: f32,
    pub flashcard: Flashcard
}

impl LogItem {
    pub fn new(card: Flashcard) -> Self {
        Self { 
            timer: MESSAGE_DURATION, 
            flashcard: card,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.timer -= dt;
    }

    pub fn can_delete(&self) -> bool {
        self.timer < 0.0
    }

    pub fn reset_timer(&mut self) {
        self.timer = MESSAGE_DURATION;
    }

    pub fn message(&self) -> String {
        format!(
            "Missed asteroid: \"{}\" = \"{}\"", 
            self.flashcard.question,
            self.flashcard.answer,
        )
    }
}
