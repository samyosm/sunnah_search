pub fn get_tokens(document: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_token = String::new();

    for ch in document.chars() {
        if ch.is_alphanumeric() {
            current_token.push(ch);
        } else if !current_token.is_empty() {
            result.push(current_token.clone());
            current_token.clear();
        }
    }

    return result;
}
