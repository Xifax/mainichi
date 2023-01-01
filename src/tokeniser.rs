use lindera::tokenizer::Tokenizer;
use lindera::{
    mode::Mode,
    tokenizer::{DictionaryConfig, TokenizerConfig},
};
use lindera::Token;


pub struct LinderaTokenizer {
    tokenizer: Tokenizer
}

/// Default implementation for tokenizer
impl <'a> LinderaTokenizer  {
    // Constructs a new instance of [`LinderaTokenizer`].
    // Note this is an associated function - no self.
    pub fn new() -> Self {
        let dictionary = DictionaryConfig {
            kind: Some(lindera::DictionaryKind::IPADIC),
            path: None,
        };

        let config = TokenizerConfig {
            dictionary,
            user_dictionary: None,
            mode: Mode::Normal,
            with_details: true,
        };

        let tokenizer = Tokenizer::new(config).unwrap();
        Self { tokenizer }
    }

    pub fn tokenize(&'a mut self, text: &'a str) -> Vec<Token> {
        self.tokenizer.tokenize(text).unwrap()
    }

}

