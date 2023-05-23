use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hadith {
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct HadithCollection {
    pub hadiths: Vec<Hadith>,
}
