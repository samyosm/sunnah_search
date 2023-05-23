use std::collections::HashMap;
use std::fs;

use crate::{
    tokenizer,
    types::{Hadith, HadithCollection},
};

pub fn get_raw_hadith_files_indexes(file_paths: Vec<&str>) -> HashMap<String, usize> {
    let mut hadiths: Vec<Hadith> = Vec::new();

    file_paths.iter().for_each(|file_path| {
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let collection: HadithCollection = serde_json::from_str(&contents).expect("this to work");

        collection
            .hadiths
            .iter()
            .for_each(|h| hadiths.push(h.clone()));
    });

    let mut index: HashMap<String, usize> = HashMap::new();
    hadiths.iter().for_each(|Hadith { text }| {
        let tokens = tokenizer::get_tokens(text);
        for token in tokens {
            index
                .entry(token.to_lowercase())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    });

    return index;
}
