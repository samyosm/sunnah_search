mod preprocessing;
mod term_counter;
mod tokenizer;
mod types;

fn main() -> Result<(), sqlite::Error> {
    term_counter::create_and_populate_term_freq_db()?;

    Ok(())
}
