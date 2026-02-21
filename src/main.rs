mod question;
mod files;
use std::path::Path; 
use crate::question::{AnswerType, QuestionType, ask_question};
//use crate::files::{}


//All the paths that I am going to use
// Users/adamporbanderwalla/.accountability
// Users/adamporbanderwalla/.accountability/data
// Users/adamporbanderwalla/.accountability/data/accountability_logs.csv
// Users/adamporbanderwalla/.accountability/data/questions.json
fn main() {
    crate::files::create_all_necessary_files();
}

