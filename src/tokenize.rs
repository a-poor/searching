

pub trait Tokenizer {
    fn tokenize(&self, text: &str) -> Vec<String>;
}

pub struct WhitespaceTokenizer;

impl Tokenizer for WhitespaceTokenizer {
    fn tokenize(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace_tokenizer() {
        let tokenizer = WhitespaceTokenizer;
        let cases = vec![
          ("hello world", vec!["hello", "world"]),
          ("hello, world!", vec!["hello,", "world!"]),
          (" hello world ", vec!["hello", "world"]),
          ("hello\tworld", vec!["hello", "world"]),
          ("hello\nworld", vec!["hello", "world"]),
          ("helloworld", vec!["helloworld"]),
          ("hello     world", vec!["hello", "world"]),
          
        ];
        for (input, expected) in cases {
          let tokens = tokenizer.tokenize(input);
          assert_eq!(tokens, expected);
        }
    }
}