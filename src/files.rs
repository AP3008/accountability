use serde; 
use std::fs::{self, create_dir, File};
use std::path::{Path, PathBuf}; 
use chrono;
use std::io;
use crate::question::Question; 
use std::ffi::OsStr;
//Ex. path: &Path = Path&::new("/Users/adamporbanderwalla/Desktop")

fn create_new_entry() -> (){
         
}

fn create_all_necessary_files(path: &Path) -> (){
    // Given a path we want to create all of the necessary files for  
}

fn check_data_exists(path: &Path) -> PathBuf{
    if path.is_dir() && (path.file_name() == Some(OsStr::new("data"))){
        return path.to_path_buf();
    }
    else if path.file_name() != Some(OsStr::new("data")){
        let mut acc_path = path.to_str().unwrap().to_string();
        acc_path.push_str("/data");
        let acc_path = Path::new(acc_path.as_str());
        create_dir(acc_path).unwrap(); 
        return acc_path.to_path_buf();  
    }
    else {
        create_dir(path); 
        return path.to_path_buf(); 
    }
}

fn create_questions_json(path: &Path) -> (){
    if path.file_name() == Some(OsStr::new("questions.json")) && path.exists(){
        return; 
    }
    //redundant check, I won't be passing in the full path; consider removing
    else if path.file_name() == Some(OsStr::new("questions.json")){
        File::create_new(path); 
    }
    else {
        let mut json_path = path.to_str().unwrap().to_string(); 
        json_path.push_str("/questions.json");
        File::create_new(Path::new(json_path.as_str())); 
    }
}

fn create_accountability_csv(path: &Path) -> (){
    check_data_exists(path);
    let mut csv_path = path.to_str().unwrap().to_string();
    csv_path.push_str("/accountability_logs.csv");
    let csv_path = Path::new(csv_path.as_str()); 
    // Now we have the path for the csv and if it exists we create it, else we exit. 

    if csv_path.exists() { return; } 

    // Creates the csv path
    File::create_new(csv_path);
}

fn store_questions(path: &Path, questions_list:Vec<Question>) -> (){
    
}

fn write_questions() -> (){
    
}


