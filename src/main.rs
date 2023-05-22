mod preprocessing;
mod tokenizer;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use tokenizer::get_tokens;

#[derive(Serialize, Deserialize)]
struct Hadith {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct HadithCollection {
    hadiths: Vec<Hadith>,
}

fn main() {
    let file_path = "./res/raw_hadiths/eng-bukhari.json";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let v: HadithCollection = serde_json::from_str(&contents).expect("valid JSON");

    for Hadith { text } in v.hadiths {
        println!("{}", text)
    }
}
