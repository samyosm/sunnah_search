use std::collections::HashMap;

use crate::{
    tokenizer,
    types::{Hadith, HadithCollection},
};

pub fn get_raw_hadith_file_index(file_path: &str) -> HashMap<String, usize> {
    let contents =
        std::fs::read_to_string(file_path).expect("Should have been able to read the file");

    let HadithCollection { hadiths } = serde_json::from_str(&contents).expect("this to work");

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
