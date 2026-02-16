use std::io::{self, Write}; 


pub enum QuestionType{
    Bool,
    Int,
    Float,
    PosNumber,
    Text,
}

pub enum AnswerType{
    Bool(bool),
    Int(i32),
    Float(f64),
    PosNumber(u32),
    Text(String),
}

pub fn ask_question(question: &str, question_type: QuestionType) -> Result<AnswerType, String>{
    loop { 
        println!("{question}");
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
        
        match question_type {
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
