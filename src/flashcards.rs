use crate::impfile;

#[derive(Clone)]
pub struct Flashcard {
    pub question: String,
    pub answer: String,
}

impl Flashcard {
    pub fn none() -> Self {
        Self {
            question: "None".to_string(),
            answer: "None".to_string(),
        }
    }

    pub fn new(q: &str, a: &str) -> Self {
        Self {
            question: q.to_string(),
            answer: a.to_string(),
        }
    }
}

//Load flashcards from an .impfile
//The question is the variable name, the answer is the variable value
fn load_flashcards_from_file(path: &str) -> Vec<Flashcard> {
    impfile::parse_file(path)
        .iter()
        .flat_map(|e| e.get_var_list())
        .map(|(question, answer)| Flashcard::new(&question, &answer))
        .collect()
}

//Load flashcards from file (assume that paths come from the arguments)
pub fn load_flashcards(paths: &[String]) -> Vec<Flashcard> {
    paths
        .iter()
        .flat_map(|path| load_flashcards_from_file(path))
        .collect()
}
