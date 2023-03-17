use tokenizers::tokenizer::{Result, Tokenizer};

fn main() -> Result<()> {
        let vocab_bytes = include_bytes!("../bert-base-uncased-vocab.txt");
        let tokenizer = Tokenizer::from_bytes(vocab_bytes).unwrap();

        let encoding = tokenizer.encode("Hey there!", false)?;
        println!("{:?}", encoding.get_tokens());
    Ok(())
}
