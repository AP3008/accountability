use std::fs::{self, create_dir, File};
use std::path::{Path}; 
use chrono;
use std::io;
use crate::question::Question; 
use std::ffi::OsStr;
//Ex. path: &Path = Path&::new("/Users/adamporbanderwalla/Desktop")

fn create_new_entry() -> (){
    
}

pub fn create_accountability_csv(path: &Path) -> (){
    check_accountability_exists(path);
    let mut csv_path = path.to_str().unwrap().to_string();
    csv_path.push_str("/accountability_logs.csv");
    let csv_path = Path::new(csv_path.as_str()); 
    // Now we have the path for the csv and if it exists we create it, else we exit. 

    if csv_path.exists() { return; } 

    // Creates the csv path
    File::create_new(csv_path);
}

fn write_questions() -> (){
    
}

fn check_accountability_exists(path: &Path) -> (){
    if path.is_dir() && (path.file_name() == Some(OsStr::new("accountability"))){

        return; 
    }
    else if path.file_name() != Some(OsStr::new("accountability")){
        let mut acc_path = path.to_str().unwrap().to_string();
        acc_path.push_str("/accountability");
        let acc_path = Path::new(acc_path.as_str());
        create_dir(acc_path).unwrap(); 
    }
    else {
        create_dir(path); 
    }
}

fn store_questions(questions_list:Vec<Question>) -> (){

}
