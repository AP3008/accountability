use std::fs::{self, create_dir};
use std::path::Path; 
use chrono;
use std::io;
use crate::question::Question; 

//Ex. path: &Path = Path&::new("/Users/adamporbanderwalla/Desktop")

fn create_new_entry() -> (){
    
}

fn create_csv(path: &Path) -> (){
    check_accountability_exists(path);
    let mut csv_path = path.to_str().unwrap().to_string();
    csv_path.push_str("/accountability_logs.csv");
    let csv_path = Path::new(csv_path.as_str()); 
    // Now we have the path for the csv and if it exists we create it, else we exit. 

    if csv_path.exists() { return; } 

    // Creates the csv path
    create_csv(csv_path);
}

fn write_questions() -> (){
    
}

fn check_accountability_exists(path: &Path) -> (){
    if path.is_dir(){
        return; 
    }
    else{
        create_dir(path).unwrap(); 
    }
}

fn store_questions(questions_list:Vec<Question>) -> (){

}
