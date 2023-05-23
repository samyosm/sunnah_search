use std::collections::HashMap;
use std::fs;

use crate::{
    tokenizer,
    types::{Hadith, HadithCollection},
};

pub fn get_raw_hadith_files_term_counts(file_paths: Vec<&str>) -> HashMap<String, usize> {
    println!("Getting term counts");
    let mut hadiths: Vec<Hadith> = Vec::new();

    file_paths.iter().for_each(|file_path| {
        println!("Getting term counts for file: {}", file_path);
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let collection: HadithCollection = serde_json::from_str(&contents).expect("this to work");

        collection
            .hadiths
            .iter()
            .for_each(|h| hadiths.push(h.clone()));
    });

    let mut term_counts: HashMap<String, usize> = HashMap::new();
    hadiths.iter().for_each(|Hadith { text }| {
        let tokens = tokenizer::get_tokens(text);
        for token in tokens {
            term_counts
                .entry(token.to_lowercase())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    });

    println!("Done getting term counts");
    return term_counts;
}

pub fn create_and_populate_term_freq_db() -> Result<(), sqlite::Error> {
    let connection = sqlite::open("./res/db/dev.db")?;

    let file_paths = vec![
        "./res/raw_hadiths/eng-malik.min.json",
        "./res/raw_hadiths/eng-bukhari.json",
        "./res/raw_hadiths/eng-nasai.min.json",
        "./res/raw_hadiths/eng-muslim.min.json",
        "./res/raw_hadiths/eng-abudawud.min.json",
        "./res/raw_hadiths/eng-ibnmajah.min.json",
        "./res/raw_hadiths/eng-tirmidhi.min.json",
    ];
    let term_counts = get_raw_hadith_files_term_counts(file_paths);
    let insertion_instruction: Vec<String> = term_counts
        .iter()
        .map(|(token, count)| format!(r#"('{}', {})"#, token, count))
        .collect();

    let query = format!(
        r#"
    CREATE TABLE term_freqs (token text primary key, freq integer);
    INSERT INTO term_freqs ("token", "freq") values {};
    "#,
        insertion_instruction.join(",")
    );
    connection.execute(query)?;

    Ok(())
}
