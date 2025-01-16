
/// A TextTransformer is a type that can...transform text.
/// It can be used to clean up text before or after being
/// tokenized.
pub trait TextTransformer {
    /// Transform the input text into a new string.
    fn transform(&self, text: &str) -> String;
}

/// A TextTransformer that converts all characters to lowercase.
pub struct LowercaseTransformer;

impl TextTransformer for LowercaseTransformer {
    fn transform(&self, text: &str) -> String {
        text.to_lowercase()
    }
}

/// A TextTransformer that removes all non-ASCII characters
pub struct AsciiFoldingTransformer;

impl TextTransformer for AsciiFoldingTransformer {
    fn transform(&self, text: &str) -> String {
        text.chars()
            .map(|c| {
                if c.is_ascii() {
                    c
                } else {
                    c.to_ascii_lowercase()
                }
            })
            .collect()
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

/// A TextTransformer that removes all non-alphanumeric characters.
pub struct AlphanumericTransformer;

impl TextTransformer for AlphanumericTransformer {
    fn transform(&self, text: &str) -> String {
        text.chars()
            .filter(|c| c.is_alphanumeric())
            .collect()
    }
}

pub struct PorterStemmerTransformer;

impl PorterStemmerTransformer {
    fn to_cv(&self, text: &str) -> Vec<char> {
        // Split into characters
        let chars = text.chars().collect::<Vec<_>>();
        if chars.len() == 0 {
            return Vec::new();
        }

        // Add the first character
        let mut cv = Vec::new();
        let mut prev = match chars[0] {
            'a' | 'e' | 'i' | 'o' | 'u' => 'V',
            _ => 'C',
        };
        cv.push(prev);

        // Loop through the rest...
        for c in chars.iter().skip(1) {
            let curr = match c {
                'a' | 'e' | 'i' | 'o' | 'u' => 'V',
                _ => 'C',
            };

            if curr != prev {
                cv.push(curr);
            }

            prev = curr;
        }

        // Done!
        cv
    }

    fn measure(&self, text: &str) -> usize {
        self.to_cv(text)
            .iter()
            .filter(|&c| *c == 'C')
            .count()
    }
}

impl TextTransformer for PorterStemmerTransformer {
    fn transform(&self, text: &str) -> String {
        let mut output = text.to_string();
        // Step 1a: Simple suffix removal
        // - SSES -> SS
        // - IES -> I
        // - SS -> SS
        // - S -> (empty)
        if output.ends_with("sses") {
            output.pop();
            output.pop();
        } else if output.ends_with("ies") {
            output.pop();
        } else if output.ends_with("ss") {
            // Do nothing
        } else if output.ends_with('s') {
            output.pop();
        }

        // Step 1b: Handle ED and ING
        // - (m>0) EED -> EE
        // - (*v*) ED -> (remove ED if the stem contains a vowel)
        // - (*v*) ING -> (remove ING if the stem contains a vowel)
        // 
        // After removing the ED or ING:
        // - AT -> ATE
        // - BL -> BLE
        // - IZ -> IZE
        // - (*d and not (*L or *S or *Z)) -> single letter
        // - (m=1 and *o) -> E
        let mut changed = false;
        let has_vowel_minus_sfx = |text: &str, sfx: &str| text
            .chars()
            .collect::<Vec<_>>()[0..text.len()-sfx.len()]
            .iter()
            .any(|c| "aeiou".contains(*c));
        if self.measure(&output) > 0 && output.ends_with("eed") {
            output.pop();
            output.pop();
            changed = true;
        } else if output.ends_with("ed") && has_vowel_minus_sfx(&output, "ed") {
            output.pop();
            output.pop();
            changed = true;
        } else if output.ends_with("ing") && has_vowel_minus_sfx(&output, "ing"){
            output.pop();
            output.pop();
            output.pop();
            changed = true;
        }

        // Step 1c:
        // - (*v*) Y -> I (if stem contains a vowel, replace Y with I)


        // Step 2:
        // - ATIONAL -> ATE
        // - TIONAL -> TION
        // - ENCI -> ENCE
        // - ANCI -> ANCE
        // - IZER -> IZE
        // - ABLI -> ABLE
        // - ALLI -> AL
        // - ENTLI -> ENT
        // - ELI -> E
        // - OUSLI -> OUS
        // - IZATION -> IZE
        // - ATION -> ATE
        // - ATOR -> ATE
        // - ALISM -> AL
        // - IVENESS -> IVE
        // - FULNESS -> FUL
        // - OUSNESS -> OUS
        // - ALITI -> AL
        // - IVITI -> IVE
        // - BILITI -> BLE


        // Step 3:
        // - (m>0) ICATE -> IC
        // - (m>0) ATIVE -> (remove)
        // - (m>0) ALIZE -> AL
        // - (m>0) ICITI -> IC
        // - (m>0) ICAL -> IC
        // - (m>0) FUL -> (remove)
        // - (m>0) NESS -> (remove)
       

        // Step 4:
        // Remove suffix if (m>1)
        // - AL
        // - ANCE
        // - ENCE
        // - ER
        // - IC
        // - ABLE
        // - IBLE
        // - ANT
        // - EMENT
        // - MENT
        // - ENT
        // - (m>1 and (*S or *T)) ION
        // - OU
        // - ISM
        // - ATE
        // - ITI
        // - OUS
        // - IVE
        // - IZE


        // Step 5a:
        // (m>1) E -> (remove)
        // (m=1 and not *o) E -> (remove if measure is 1 and doesn't end in cvc)


        // Step 5b:
        // (m>1 and *d and *L) E -> single letter (eg "LL" -> "L")

        todo!()
    }
}

