mod preprocessing;
mod tokenizer;

use tokenizer::get_tokens;

fn main() {
    let document = r#"I heard Allah's Messenger (ﷺ) saying, "The reward of deeds depends upon the intentions and every person will get the reward according to what he has intended. So whoever emigrated for worldly benefits or for a woman to marry, his emigration was for what he emigrated for.""#;
    let tokens = get_tokens(document);

    println!("{:?}", tokens);
}
