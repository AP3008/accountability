use crate::question::{self, AnswerType, DailyEntry, Question};
use serde_json;
use std::env::home_dir;
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions, create_dir};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub fn create_new_entry(path: &Path, daily_entry: DailyEntry) -> () {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();

    let is_empty = match fs::metadata(path) {
        Ok(metadata) => metadata.len() == 0,
        Err(_) => true,
    };

    let mut csv_writer = csv::WriterBuilder::new()
        .has_headers(is_empty)
        .from_writer(file);
    csv_writer.serialize(&daily_entry).unwrap();
    csv_writer.flush().unwrap();
}

pub fn create_all_necessary_files() -> PathBuf {
    // We want to create all of this inside of .accountability in the users home dir
    let path: PathBuf = create_dotfile();

    // Create a data folder
    let data_path: PathBuf = check_data_exists(&path);
    println!("~/.accountability/data : has been created");

    create_accountability_csv(&data_path);
    println!("~/.accountability/data/accountability_logs.csv : has been created");

    create_questions_json(&data_path);
    println!("~/.accountability/data/questions.json : has been created");

    // ~/.accountability
    return data_path;
}

fn check_data_exists(path: &Path) -> PathBuf {
    if path.is_dir() && (path.file_name() == Some(OsStr::new("data"))) {
        return path.to_path_buf();
    } else if path.file_name() != Some(OsStr::new("data")) {
        let mut acc_path = path.to_str().unwrap().to_string();
        acc_path.push_str("/data");
        let acc_path = Path::new(acc_path.as_str());
        if !(acc_path.exists()) {
            create_dir(acc_path).unwrap();
        }
        return acc_path.to_path_buf();
    } else {
        create_dir(path);
        return path.to_path_buf();
    }
}

fn create_questions_json(path: &Path) {
    let mut json_path = path.to_str().unwrap().to_string();
    json_path.push_str("/questions.json");
    let json_path = Path::new(&json_path);

    if !json_path.exists() {
        fs::write(json_path, "[]").unwrap();
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
    let json = serde_json::to_string_pretty(questions_list).unwrap();
    fs::write(path, json);
}

pub fn load_questions(path: &Path) -> Vec<Question> {
    let data = fs::read_to_string(path).unwrap_or_default();
    let trimmed = data.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }
    serde_json::from_str(trimmed).unwrap()
}
pub fn write_questions(path: &Path, question: Question) -> () {
    let mut questions = load_questions(path);
    questions.push(question);
    store_questions(path, &questions);
}

pub fn list_questions(path: &Path) -> () {
    let questions = load_questions(path);
    for (index, value) in questions.iter().enumerate() {
        println!("Question {index}: {}", value.question);
    }
}

pub fn delete_question(path: &Path) -> () {
    let mut questions_list = load_questions(path);
    loop {
        io::stdout().flush().unwrap();
        let mut input = String::new();
        println!("Choose a question to remove (or q): ");
        match io::stdin().read_line(&mut input) {
            Ok(i) => i,
            Err(_e) => {
                return;
            }
        };
        let trimmed = input.trim();
        if trimmed.eq_ignore_ascii_case("q") {
            return;
        }
        let input: i32 = trimmed.parse().expect("Parsing failed."); if ((input as usize) > questions_list.len()) || (input < 0) {
            continue;
        }
        questions_list.remove(input as usize);
        break;
    }
    store_questions(path, &questions_list);
}

pub fn list_answers(path: &Path) -> (){
    let mut rdr = csv::Reader::from_path(path).unwrap();    
    for res in rdr.records(){
        let res = res.unwrap().clone(); 
        println!("----------------");
        println!("Date: {}", res[0].to_string());
        println!("Question: {}", res[1].to_string());
        println!("Answer: {}", res[2].to_string()); 
        println!("----------------");
        print!("\n\nNext? (n): ");
        std::io::stdout().flush().unwrap(); 
        let mut input = String::new(); 
        std::io::stdin().read_line(&mut input); 
        let input = input.trim(); 
        match input.to_lowercase().as_str(){
            "n" | "next" | "nex" | "ne" => { continue; } 
            _ => { break }
        }
    }
}
