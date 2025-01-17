
/// A TextTransformer is a type that can...transform text.
/// It can be used to clean up text before or after being
/// tokenized.
pub trait TextTransformer {
    /// Transform the input text into a new string.
    fn transform(&self, text: &str) -> String;
}

pub struct NoOpTransformer;

impl TextTransformer for NoOpTransformer {
    fn transform(&self, text: &str) -> String {
        text.to_string()
    }
}

/// A TextTransformer that converts all characters to lowercase.
pub struct LowercaseTransformer;

impl TextTransformer for LowercaseTransformer {
    fn transform(&self, text: &str) -> String {
        text.to_lowercase()
    }
}

// /// A TextTransformer that removes all non-ASCII characters
// pub struct AsciiFoldingTransformer;
//
// impl TextTransformer for AsciiFoldingTransformer {
//     fn transform(&self, text: &str) -> String {
//         todo!();
//     }
// }

/// A TextTransformer that removes punctuation characters.
pub struct RemovePunctuationTransformer;

impl TextTransformer for RemovePunctuationTransformer {
    fn transform(&self, text: &str) -> String {
        text.chars()
            .filter(|c| !c.is_ascii_punctuation())
            .collect()
    }
}

/// A TextTransformer that removes all non-alphanumeric characters.
pub struct AlphanumericTransformer;

impl TextTransformer for AlphanumericTransformer {
    fn transform(&self, text: &str) -> String {
        text.chars()
            .filter(|c| c.is_alphanumeric())
            .collect()
    }
} 

