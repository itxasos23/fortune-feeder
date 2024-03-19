use home;
use serde_derive::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct Message {
    sender: String,
    text: String,
    date: String,
}

fn load_messages(filename: &str) -> Vec<Message> {
    let contents = fs::read_to_string(&filename).unwrap();
    let messages: Vec<Message> = serde_json::from_str(&contents).unwrap();
    dbg!(&messages);
    messages
}

fn get_save_file() -> PathBuf {
    let mut save_file_path = match home::home_dir() {
        Some(path) => path,
        None => panic!("Impossible to get your home dir!"),
    };
    save_file_path.push(".local");
    save_file_path.push("share");
    save_file_path.push("fortune");
    save_file_path.push("telegram_pinned");
    save_file_path
}

fn save_file(filename: &PathBuf, payload: &str) {
    dbg!(filename);
    fs::write(filename, payload).unwrap();
}

fn create_dat_file(filename: &PathBuf) {
    Command::new("strfile")
        .arg(filename)
        .output()
        .expect("Failed to execute command");
}

fn main() {
    let messages = load_messages("2024-03-19.json");
    let serialized_rows: Vec<String> = messages
        .iter()
        .map(|msg| format!("{} \n --- \n {} on {}", msg.text, msg.sender, msg.date))
        .collect();
    let payload = format!("{}", serialized_rows.join("\n%\n"));

    let save_file_path = get_save_file();

    save_file(&save_file_path, &payload);
    create_dat_file(&save_file_path);
}
