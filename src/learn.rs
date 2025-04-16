use crate::flashcards::Flashcard;
use rand::seq::SliceRandom;
use std::collections::{HashSet, VecDeque};

//How long should we display the correct answer for the user?
const DISPLAY_CORRECT_ANS_TIMER: f32 = 1.5;

pub struct LearnState {
    flashcards: Vec<Flashcard>,
    pub mcq: VecDeque<Flashcard>,
    pub frq: VecDeque<Flashcard>,
    pub size: usize,
    pub mcq_ans: Vec<String>,
    timer: f32,
    pub answer: String,
    submitted: bool,
}

impl LearnState {
    pub fn new(cards: &[Flashcard]) -> Self {
        let mut rng = rand::rng();

        let mut shuffled_mcq = cards.to_vec();
        shuffled_mcq.shuffle(&mut rng);

        let mut shuffled_frq = cards.to_vec();
        shuffled_frq.shuffle(&mut rng);
        shuffled_frq.extend(shuffled_frq.clone());

        let sz = shuffled_mcq.len() + shuffled_frq.len();

        Self {
            flashcards: cards.to_vec(),
            mcq: VecDeque::from(shuffled_mcq),
            frq: VecDeque::from(shuffled_frq),
            size: sz,
            mcq_ans: vec![],
            timer: 0.0,
            answer: String::new(),
            submitted: false,
        }
    }

    pub fn empty() -> Self {
        Self {
            flashcards: vec![],
            mcq: VecDeque::new(),
            frq: VecDeque::new(),
            size: 0,
            mcq_ans: vec![],
            timer: 0.0,
            answer: String::new(),
            submitted: false,
        }
    }

    pub fn set_mcq_ans(&mut self) {
        self.mcq_ans.clear();

        if self.mcq.is_empty() {
            return;
        }

        let correct = rand::random::<u32>() % 4;
        let flashcard = self.get_flashcard().unwrap_or(Flashcard::none());
        let mut skip = HashSet::new();
        skip.insert(flashcard.answer.clone());
        for i in 0..4 {
            //Generate the correct answer
            if i == correct {
                self.mcq_ans.push(flashcard.answer.clone());
                continue;
            }

            //Generate the incorrect answers
            let mut index = rand::random::<u32>() as usize % self.flashcards.len();
            let mut count = 0;
            while skip.contains(&self.flashcards[index].answer) && count < self.flashcards.len() {
                index += 1;
                index %= self.flashcards.len();
                count += 1;
            }
            self.mcq_ans.push(self.flashcards[index].answer.clone());
            skip.insert(self.flashcards[index].answer.clone());
        }
    }

    pub fn get_flashcard(&self) -> Option<Flashcard> {
        if !self.mcq.is_empty() {
            self.mcq.front().cloned()
        } else {
            self.frq.front().cloned()
        }
    }

    pub fn percent(&self) -> f32 {
        let left = self.mcq.len() + self.frq.len();
        1.0 - left as f32 / self.size as f32
    }

    pub fn submit(&mut self, ans: &str) {
        if self.submitted {
            return;
        }

        self.answer = ans.to_string();
        self.submitted = true;
        self.timer = DISPLAY_CORRECT_ANS_TIMER;
    }

    pub fn display_correct(&self) -> bool {
        self.timer > 0.0 && self.submitted
    }

    pub fn update(&mut self, dt: f32) {
        if self.submitted {
            self.timer -= dt;
        }

        if self.timer <= 0.0 && self.submitted {
            let card = self.get_flashcard().unwrap_or(Flashcard::none());
            if !self.mcq.is_empty() {
                //Handle multiple choice
                if self.answer == card.answer {
                    self.mcq.pop_front();
                    self.set_mcq_ans();
                } else {
                    let card = self.mcq.pop_front();
                    if let Some(card) = card {
                        self.mcq.push_back(card);
                    }
                    self.set_mcq_ans();
                }
            } else {
                //Handle free response
                if self.answer == card.answer {
                    self.frq.pop_front();
                } else {
                    let card = self.frq.pop_front();
                    if let Some(card) = card {
                        self.frq.push_back(card);
                    }
                }
            }
            self.answer.clear();
            self.submitted = false;
        }
    }
}
