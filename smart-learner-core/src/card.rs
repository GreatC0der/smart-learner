use crate::field::Field;
use crate::result::Result;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub front: Field,
    pub back: Field,
    pub current_repeat_in: u64,
    pub repeat_in: u64,
}

impl Card {
    pub fn new(front: Field, back: Field) -> Self {
        Self {
            front,
            back,
            current_repeat_in: 0,
            repeat_in: 1,
        }
    }
    pub fn review(&mut self, result: Result) {
        match result {
            Result::Easy => {
                self.current_repeat_in = self.repeat_in;
                self.repeat_in *= 2;
            }
            Result::Difficult => {
                self.repeat_in *= 2;
            }
            Result::Wrong => {
                if self.repeat_in > 1 {
                    self.repeat_in /= 2;
                }
                self.current_repeat_in = self.repeat_in;
            }
        }
    }
}
