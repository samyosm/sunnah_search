use std::collections::HashSet;

pub fn get_tokens(document: &str) -> HashSet<String> {
    let mut result = HashSet::new();
    let mut current_token = String::new();

    for ch in document.chars() {
        if ch.is_alphanumeric() {
            current_token.push(ch);
        } else if !current_token.is_empty() {
            result.insert(current_token.clone());
            current_token.clear();
        }
    }

    return result;
}
