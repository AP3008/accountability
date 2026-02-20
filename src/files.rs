use serde;
use std::env::home_dir; 
use std::fs::{self, create_dir, File};
use std::path::{Path, PathBuf}; 
use chrono;
use std::io;
use crate::question::Question; 
use std::ffi::OsStr;


// HANDLE ERRORS FOR WHEN FILE DOES EXIST.

fn create_new_entry() -> (){
         
}

pub fn create_all_necessary_files() -> (){
    // We want to create all of this inside of .accountability in the users home dir
    let path: PathBuf = create_dotfile(); 
    
    // Create a data folder
    let data_path: PathBuf = check_data_exists(&path);
    println!("~/.accountability/data : has been created");
    
    create_accountability_csv(&data_path);
    println!("~/.accountability/data/accountability_logs.csv : has been created");

    create_questions_json(&data_path);
    println!("~/.accountability/data/questions.json : has been created");
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
    let mut csv_path = path.to_str().unwrap().to_string();
    csv_path.push_str("/accountability_logs.csv");
    let csv_path = Path::new(csv_path.as_str()); 
    // Now we have the path for the csv and if it exists we create it, else we exit. 

    if csv_path.exists() { return; } 

    // Creates the csv path
    File::create_new(csv_path);
}

pub fn create_dotfile() -> PathBuf {
    let home = home_dir().unwrap(); 
    let mut path_str = home.to_str().unwrap().to_string();
    path_str.push_str("/.accountability");
    let acc_path = Path::new(path_str.as_str());
    if !(acc_path.exists() || acc_path.is_dir()){
        create_dir(acc_path); 
    }
    return acc_path.to_path_buf(); 
}

fn store_questions(path: &Path, questions_list:Vec<Question>) -> (){
    
}

fn write_questions() -> (){
    
}


