mod question;
mod files;
use std::{io::{Write, stdout}, path::Path}; 
use crate::{files::delete_question, question::{AnswerType, QuestionType, ask_question}};
use std::path::PathBuf;

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
        print!("Pick a number (or q): "); 
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input){
            Ok(i) => i,
            Err(_e) => { return; }
        };
        let trimmed = input.trim();
        if trimmed.eq_ignore_ascii_case("q") { break; }

        let input: i32 = match trimmed.parse() {
            Ok(n) => n,
            Err(_) => {
                continue;
            }
        };
        match input{
            1 => {
                let question_list = crate::files::load_questions(&path_json);
                for value in question_list.iter(){
                    let q = &value.question; 
                    let res = match ask_question(value){
                        Ok(ans) => { ans },
                        Err(e) => { continue; },
                    };
                    let a = match res {
                        AnswerType::Int(i) => { i.to_string() },
                        AnswerType::Bool(b) => { b.to_string() },
                        AnswerType::Text(s) => { s.to_string() },
                        AnswerType::Float(f) => { f.to_string() },
                        AnswerType::PosNumber(u) => { u.to_string() },
                    }; 
                    let daily_entry = crate::question::answer_to_entry(q.as_str(), a.as_str()); 
                    crate::files::create_new_entry(path_csv, daily_entry);
                }
            },
            2 => {
                let new_question = match crate::question::create_new_question(){
                    Ok(q) => { q },
                    Err(e) => { 
                        println!("Error: {e}");
                        continue; 
                    }
                };
                crate::files::write_questions(&path_json, new_question);
            },
            3 => { delete_question(&path); } 
            4 => {  }
            _ => { continue; }
        }
    }
}

