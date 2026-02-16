use std::io; 


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

pub fn ask_question(question: &str, question_type: AnswerType) -> Result<AnswerType>{
    
}
