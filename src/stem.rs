use crate::transform::TextTransformer;

pub struct PorterStemmerTransformer;

impl PorterStemmerTransformer {
    fn is_vowel(c: char) -> bool {
        ['a', 'e', 'i', 'o', 'u'].contains(&c)
    }

    /// Check if the character at given index is a consonant
    fn is_consonant(word: &[u8], i: usize) -> bool {
        let c = word[i] as char;
        if Self::is_vowel(c) {
            return false;
        }
        if c == 'y' {
            // If y is preceded by a consonant, it's a vowel
            return i == 0 || Self::is_consonant(word, i - 1);
        }
        true
    }

    /// Calculate the measure (m) of a word
    fn measure(stem: &[u8]) -> usize {
        let mut count = 0;
        let mut is_cons = true;
        
        for i in 0..stem.len() {
            let new_is_cons = Self::is_consonant(stem, i);
            if !new_is_cons && is_cons {
                count += 1;
            }
            is_cons = new_is_cons;
        }
        
        count
    }

    /// Check if the stem contains at least one vowel
    fn has_vowel(stem: &[u8]) -> bool {
        for i in 0..stem.len() {
            if !Self::is_consonant(stem, i) {
                return true;
            }
        }
        false
    }

    /// Check if stem ends with a double consonant
    fn ends_with_double_consonant(stem: &[u8]) -> bool {
        if stem.len() < 2 {
            return false;
        }
        let last = stem.len() - 1;
        stem[last] == stem[last - 1] && Self::is_consonant(stem, last)
    }

    /// Check if stem ends with CVC pattern
    fn ends_cvc(stem: &[u8]) -> bool {
        if stem.len() < 3 {
            return false;
        }
        let last = stem.len() - 1;
        let last_char = stem[last] as char;
        
        Self::is_consonant(stem, last)
            && !Self::is_consonant(stem, last - 1)
            && Self::is_consonant(stem, last - 2)
            && !matches!(last_char, 'w' | 'x' | 'y')
    }

    /// Step 1a of the Porter Stemmer algorithm
    fn step1a(word: &[u8]) -> Vec<u8> {
        let word_str = std::str::from_utf8(word).unwrap();
        
        if word_str.ends_with("sses") {
            return word[..word.len() - 2].to_vec();
        } else if word_str.ends_with("ies") {
            return [&word[..word.len() - 3], b"i"].concat();
        } else if word_str.ends_with("ss") {
            return word.to_vec();
        } else if word_str.ends_with('s') {
            return word[..word.len() - 1].to_vec();
        }
        
        word.to_vec()
    }

    /// Step 1b of the Porter Stemmer algorithm
    fn step1b(word: &[u8]) -> Vec<u8> {
        let word_str = std::str::from_utf8(word).unwrap();
        
        if word_str.ends_with("eed") {
            let stem = &word[..word.len() - 3];
            if Self::measure(stem) > 0 {
                return [stem, b"ee"].concat();
            }
        } else if word_str.ends_with("ed") {
            let stem = &word[..word.len() - 2];
            if Self::has_vowel(stem) {
                return Self::step1b_cleanup(stem);
            }
        } else if word_str.ends_with("ing") {
            let stem = &word[..word.len() - 3];
            if Self::has_vowel(stem) {
                return Self::step1b_cleanup(stem);
            }
        }
        
        word.to_vec()
    }

    /// Helper function for step1b cleanup
    fn step1b_cleanup(stem: &[u8]) -> Vec<u8> {
        let stem_str = std::str::from_utf8(stem).unwrap();
        
        if stem_str.ends_with("at") || stem_str.ends_with("bl") || stem_str.ends_with("iz") {
            return [stem, b"e"].concat();
        } else if Self::ends_with_double_consonant(stem) 
            && !stem_str.ends_with('l')
            && !stem_str.ends_with('s')
            && !stem_str.ends_with('z') {
            return stem[..stem.len() - 1].to_vec();
        } else if Self::measure(stem) == 1 && Self::ends_cvc(stem) {
            return [stem, b"e"].concat();
        }
        
        stem.to_vec()
    }

    /// Step 1c of the Porter Stemmer algorithm
    fn step1c(word: &[u8]) -> Vec<u8> {
        let word_str = std::str::from_utf8(word).unwrap();
        
        if word_str.ends_with('y') {
            let stem = &word[..word.len() - 1];
            if Self::has_vowel(stem) {
                return [stem, b"i"].concat();
            }
        }
        
        word.to_vec()
    }

    /// Step 2 of the Porter Stemmer algorithm
    fn step2(word: &[u8]) -> Vec<u8> {
        let word_str = std::str::from_utf8(word).unwrap();
        let replacements = [
            ("ization", "ize"),
            ("ational", "ate"),
            ("tional", "tion"),
            ("enci", "ence"),
            ("anci", "ance"),
            ("izer", "ize"),
            ("abli", "able"),
            ("alli", "al"),
            ("entli", "ent"),
            ("eli", "e"),
            ("ousli", "ous"),
            ("ation", "ate"),
            ("ator", "ate"),
            ("alism", "al"),
            ("iveness", "ive"),
            ("fulness", "ful"),
            ("ousness", "ous"),
            ("aliti", "al"),
            ("iviti", "ive"),
            ("biliti", "ble"),
        ];

        for (suffix, replacement) in replacements.iter() {
            if word_str.ends_with(suffix) {
                let stem = &word[..word.len() - suffix.len()];
                if Self::measure(stem) > 0 {
                    return [stem, replacement.as_bytes()].concat();
                }
            }
        }
        
        word.to_vec()
    }

    /// Step 3 of the Porter Stemmer algorithm
    fn step3(word: &[u8]) -> Vec<u8> {
        let word_str = std::str::from_utf8(word).unwrap();
        let replacements = [
            ("icate", "ic"),
            ("ative", ""),
            ("iciti", "ic"),
            ("ical", "ic"),
            ("ful", ""),
            ("ness", ""),
        ];

        for (suffix, replacement) in replacements.iter() {
            if word_str.ends_with(suffix) {
                let stem = &word[..word.len() - suffix.len()];
                if Self::measure(stem) > 0 {
                    return [stem, replacement.as_bytes()].concat();
                }
            }
        }
        
        word.to_vec()
    }

    /// Step 4 of the Porter Stemmer algorithm
    fn step4(word: &[u8]) -> Vec<u8> {
        let word_str = std::str::from_utf8(word).unwrap();
        let suffixes = [
            "al", "ance", "ence", "er", "ic", "able", "ible", "ant",
            "ement", "ment", "ent", "ion", "ou", "ism", "ate", "iti",
            "ous", "ive", "ize"
        ];

        for &suffix in suffixes.iter() {
            if word_str.ends_with(suffix) {
                let stem = &word[..word.len() - suffix.len()];
                if suffix == "ion" {
                    if stem.len() >= 1 {
                        let last_char = stem[stem.len() - 1] as char;
                        if (last_char == 's' || last_char == 't') && Self::measure(stem) > 1 {
                            return stem.to_vec();
                        }
                    }
                } else if Self::measure(stem) > 1 {
                    return stem.to_vec();
                }
            }
        }
        
        word.to_vec()
    }

    /// Step 5a of the Porter Stemmer algorithm
    fn step5a(word: &[u8]) -> Vec<u8> {
        let word_str = std::str::from_utf8(word).unwrap();
        
        if word_str.ends_with('e') {
            let stem = &word[..word.len() - 1];
            let m = Self::measure(stem);
            if m > 1 || (m == 1 && !Self::ends_cvc(stem)) {
                return stem.to_vec();
            }
        }
        
        word.to_vec()
    }

    /// Step 5b of the Porter Stemmer algorithm
    fn step5b(word: &[u8]) -> Vec<u8> {
        if Self::measure(word) > 1 
            && Self::ends_with_double_consonant(word) 
            && std::str::from_utf8(word).unwrap().ends_with('l') {
            return word[..word.len() - 1].to_vec();
        }
        
        word.to_vec()
    }

    /// Main entry point for stemming a single word
    fn stem_word(word: &str) -> String {
        if word.len() <= 2 {
            return word.to_string();
        }

        let mut current = word.to_lowercase().into_bytes();
        current = Self::step1a(&current);
        current = Self::step1b(&current);
        current = Self::step1c(&current);
        current = Self::step2(&current);
        current = Self::step3(&current);
        current = Self::step4(&current);
        current = Self::step5a(&current);
        current = Self::step5b(&current);

        String::from_utf8(current).unwrap()
    }
}

impl TextTransformer for PorterStemmerTransformer {
    fn transform(&self, text: &str) -> String {
      Self::stem_word(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_stemming() {
        let stemmer = PorterStemmerTransformer;
        
        let test_cases = [
            ("caresses", "caress"),
            ("ponies", "poni"),
            ("ties", "ti"),
            ("caress", "caress"),
            ("cats", "cat"),
            ("feed", "feed"),
            ("agreed", "agre"),
            ("plastered", "plaster"),
            ("bled", "bled"),
            ("motoring", "motor"),
            ("sing", "sing"),
            ("conflated", "conflat"),
            ("troubled", "troubl"),
            ("sized", "size"),
            ("hopping", "hop"),
            ("failing", "fail"),
            ("filing", "file"),
            ("disabled", "disabl"),
            ("matting", "mat"),
            ("mating", "mate"),
            ("meeting", "meet"),
            ("milling", "mill"),
            ("messing", "mess"),
            ("meetings", "meet"),
            ("fishy", "fishi"),
            ("by", "by"),
            ("realization", "realiz"),
            ("ion", "ion"),
            ("able", "abl"),
            ("feare", "fear"),
            ("mate", "mate"),
            ("hell", "hell")
        ];

        for (input, expected) in test_cases.iter() {
            assert_eq!(stemmer.transform(input), *expected, "Input: {}", input);
        }
    }
}


