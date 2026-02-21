mod question;
mod files;
use std::{io::{Write, stdout}, path::Path}; 
use crate::question::{AnswerType, QuestionType, ask_question};
use std::path::PathBuf;

//All the paths that I am going to use
// Users/adamporbanderwalla/.accountability
// Users/adamporbanderwalla/.accountability/data
// Users/adamporbanderwalla/.accountability/data/accountability_logs.csv
// Users/adamporbanderwalla/.accountability/data/questions.json
fn main() {
    //~/.accountability/data path
    let path: PathBuf = crate::files::create_all_necessary_files();
    let mut path_json = path.to_str().unwrap().to_string();  
    let mut path_csv = path_json.clone();
    path_json.push_str("/questions.json");
    path_csv.push_str("/accountability_logs.csv");
    let path_json = Path::new(path_json.as_str());
    let path_csv = Path::new(path_csv.as_str()); 


    println!("What do you need to be held accountable for?");
    println!("\t1. Record Todays Answers");
    println!("\t2. Add Questions");
    println!("\t3. Delete Questions");
    println!("\t4. List Answers");
    
    loop {
        println!("Pick a number (or q): "); 
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input){
            Ok(i) => i,
            Err(_e) => { return; }
        };
        if input.trim() == "q" { break; }
        let input: i32 = input.trim().parse().expect("Parsing Failed.\n");
        match input{
            1 => {
                let question_list = crate::files::load_questions(&path);
            }
            _ => { continue; }
        }
    }
}

