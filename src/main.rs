mod question;
mod files;
use std::path::Path; 
use crate::question::{AnswerType, QuestionType, ask_question};
//use crate::files::{}

fn main() {
    let path: &Path = Path::new("/Users/adamporbanderwalla/Desktop/accountability");
    crate::files::create_accountability_csv(path);
}

