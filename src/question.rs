use std::{error::Error, io::{self, Write}}; 
use serde::{Serialize, Deserialize};

use crate::question;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question{
    question: String, 
    question_type: QuestionType,
    title: String,
}

//Enum to identify the expected answer type from a question. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionType{
    Bool,
    Int,
    Float,
    PosNumber,
    Text,
}

//Enum to identify the type the answer should be
pub enum AnswerType{
    Bool(bool),
    Int(i32),
    Float(f64),
    PosNumber(u32),
    Text(String),
}

pub fn ask_question(question: &Question) -> Result<AnswerType, String>{
    loop { 
        println!("{}",question.question);
        match io::stdout().flush(){
                Ok(e) => { e },
                Err(_e) => { println!("Error Flushing"); }
            };

        let mut input = String::new();
        match io::stdin().read_line(&mut input){
                Ok(i) => { i },
                Err(e) => { 
                    println!("Error: {e}");
                    return Err("{e}".to_string()); 
                }
            };
        let input = input.trim(); 
        if input.eq_ignore_ascii_case("q"){
            return Err("Good bye!".to_string());
            }
        
        // Now we need to make sure input matches the type
        
        match question.question_type {
           QuestionType::Int => {
               match input.parse::<i32>(){ // We want to specify i32
                    Ok(num) => return Ok(AnswerType::Int(num)),
                    Err(_e) => continue
               } 
           },
           QuestionType::Float => {
               match input.parse::<f64>(){
                    Ok(num) => return Ok(AnswerType::Float(num)),
                    Err(_e) => continue
               } 
           },
           QuestionType::PosNumber => {
               match input.parse::<u32>(){
                    Ok(num) => return Ok(AnswerType::PosNumber(num)),
                    Err(_e) => continue
               }
           }, 
           QuestionType::Text => {
                // Just need to make sure it is not empty.
                if input.is_empty() { 
                    println!("Please write something!");
                    continue;
                }
                return Ok(AnswerType::Text(input.to_string())); 
           }, 
           QuestionType::Bool => {
                match input.to_lowercase().as_str() {
                "y" | "yes" | "true" | "1" | "t" => return Ok(AnswerType::Bool(true)),
                "n" | "no" | "false" | "0" | "f" => return Ok(AnswerType::Bool(false)),
                _ => {
                    println!("Enter y/n, 1/0, t/f");
                    continue;
                    }
                }
           }
        };
    }
}

pub fn create_new_question() -> Result<Question, std::io::Error>{
    loop {
        println!("What would you like your next Question to be: "); 
        io::stdout().flush().unwrap(); 
        let mut question_q = String::new(); 
        match io::stdin().read_line(&mut question_q){
            Ok(i) => { i } ,
            Err(e) => {
                println!("Error: {e}");
                return Err(e);
            }
        };
        let question_q = question_q.trim();

        let mut question_t: QuestionType;
        
        loop{
            println!("What would type do you want the answer to be: ");
            io::stdout().flush().unwrap();
            let mut question_type = String::new();
            match io::stdin().read_line(&mut question_type){
                Ok(i) => { i },
                Err(e) => { return Err(e); }
            };
            let question_type = question_type.trim().to_lowercase();  
            match question_type.as_str(){
                "int" | "integer" | "i" => { 
                    question_t = QuestionType::Int;
                    break; 
                    },
                "bool" | "b" => {
                    question_t = QuestionType::Bool; 
                    break;
                },
                "float" | "f"=> {
                    question_t = QuestionType::Float; 
                    break;
                },
                "posnumber" | "unsigned" | "u" => {
                    question_t = QuestionType::PosNumber;
                    break;
                },
                "s" | "text" | "string" | "str" => {
                    question_t = QuestionType::Text;
                },
                _ => { 
                    println!("Please eneter a valid option");
                    println!("- int");
                    println!("- float");
                    println!("- unsigned");
                    println!("- string");
                    continue
                }
            }
        } 
        io::stdout().flush().unwrap(); 
        let mut question_title = String::new(); 
        println!("What do you want the title of this question to be: "); 
        match io::stdin().read_line(&mut question_title){
            Ok(i) => { i },
            Err(e) => { return Err(e)}
        };
        let question_title = question_title.trim(); 
        let question: Question = Question {
            question : question_q.to_string(),
            question_type : question_t,
            title : question_title.to_string()
        };
        return Ok(question)
    }
}
