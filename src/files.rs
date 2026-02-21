use crate::question::{AnswerType, Question};
use chrono;
use serde;
use serde_json;
use std::env::home_dir;
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions, create_dir};
use std::io;
use std::path::{Path, PathBuf};

// HANDLE ERRORS FOR WHEN FILE DOES EXIST.

fn create_new_entry(path: &Path, answer: AnswerType) -> () {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)
        .unwrap();
    let mut csv_writer = csv::Writer::from_writer(file); 
    
}

pub fn create_all_necessary_files() -> () {
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

fn check_data_exists(path: &Path) -> PathBuf {
    if path.is_dir() && (path.file_name() == Some(OsStr::new("data"))) {
        return path.to_path_buf();
    } else if path.file_name() != Some(OsStr::new("data")) {
        let mut acc_path = path.to_str().unwrap().to_string();
        acc_path.push_str("/data");
        let acc_path = Path::new(acc_path.as_str());
        if !(acc_path.exists()){
            create_dir(acc_path).unwrap();
        }
        return acc_path.to_path_buf();
    } else {
        create_dir(path);
        return path.to_path_buf();
    }
}

fn create_questions_json(path: &Path) -> () {
    if path.file_name() == Some(OsStr::new("questions.json")) && path.exists() {
        return;
    }
    //redundant check, I won't be passing in the full path; consider removing
    else if path.file_name() == Some(OsStr::new("questions.json")) {
        if !path.exists(){
            File::create_new(path);
        }
    } else {
        let mut json_path = path.to_str().unwrap().to_string();
        json_path.push_str("/questions.json");
        if !path.exists(){
            File::create_new(Path::new(json_path.as_str()));
        }
    }
}

fn create_accountability_csv(path: &Path) -> () {
    let mut csv_path = path.to_str().unwrap().to_string();
    csv_path.push_str("/accountability_logs.csv");
    let csv_path = Path::new(csv_path.as_str());
    // Now we have the path for the csv and if it exists we create it, else we exit.

    if csv_path.exists() {
        return;
    }

    // Creates the csv path
    File::create_new(csv_path);
}

pub fn create_dotfile() -> PathBuf {
    let home = home_dir().unwrap();
    let mut path_str = home.to_str().unwrap().to_string();
    path_str.push_str("/.accountability");
    let acc_path = Path::new(path_str.as_str());
    if !(acc_path.exists() || acc_path.is_dir()) {
        create_dir(acc_path);
    }
    return acc_path.to_path_buf();
}

fn store_questions(path: &Path, questions_list: &[Question]) -> () {
    // We serialize the vector of questions so we can write it to our file
    // Because I am the only one calling the function in another function we don't need to check if
    // file path exists.
    let json = serde_json::to_string_pretty(path).unwrap();
    fs::write(path, json);
}

fn write_questions() -> () {}
