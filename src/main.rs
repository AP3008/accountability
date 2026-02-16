mod question;

use crate::question::{AnswerType, QuestionType, ask_question};

fn main() {
    let question = "How was your day? ";
    let qt = QuestionType::Float;
    let ans = ask_question(question, qt).unwrap(); 
    match ans {
        AnswerType::Float(t) => println!("You said: {t}"),
        _ => println!("Unexpected Type")
    }; 
}

