
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

/// A TextTransformer that removes punctuation characters.
pub struct RemovePunctuationTransformer;

impl TextTransformer for RemovePunctuationTransformer {
    fn transform(&self, text: &str) -> String {
        text.chars()
            .filter(|c| !c.is_ascii_punctuation())
            .collect()
    }
}

pub struct PunctuationToSpaceTransformer;

impl TextTransformer for PunctuationToSpaceTransformer {
    fn transform(&self, text: &str) -> String {
        text.chars()
            .map(|c| if c.is_ascii_punctuation() { ' ' } else { c })
            .collect()
    }
}

/// A TextTransformer that removes all non-alphanumeric characters
/// and applies (basic) ascii folding.
pub struct AlphanumericTransformer;

impl TextTransformer for AlphanumericTransformer {
    fn transform(&self, text: &str) -> String {
        text.chars()
          .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => c,
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'á' | 'à' | 'â' | 'ä' => 'a',
            'í' | 'ì' | 'î' | 'ï' => 'i',
            'ó' | 'ò' | 'ô' | 'ö' => 'o',
            'ú' | 'ù' | 'û' | 'ü' => 'u',
            _ => ' ',
          })
          .collect()
    }
}

pub struct PreProcessTransformer;

impl TextTransformer for PreProcessTransformer {
    fn transform(&self, text: &str) -> String {
      let text = LowercaseTransformer.transform(text);
      let text = AlphanumericTransformer.transform(&text);
      let text = PunctuationToSpaceTransformer.transform(&text);
      text
    }
}

