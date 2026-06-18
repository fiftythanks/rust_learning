const CONSONANTS: [char; 65] = [
    'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', 'F', 'f', 'G', 'g', 'H', 'h', 'J', 'j', 'K', 'k', 'L',
    'l', 'M', 'm', 'N', 'n', 'Ñ', 'ñ', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'ß', 'T', 't', 'V',
    'v', 'W', 'w', 'X', 'x', 'Z', 'z', 'Ź', 'ź', 'Ż', 'ż', 'Ž', 'ž', 'Ś', 'ś', 'Ş', 'ş', 'Š', 'š',
    'Ć', 'ć', 'Č', 'č', 'Ď', 'ď', 'Ť', 'ť',
];

const VOWELS: [char; 58] = [
    'A', 'a', 'Á', 'á', 'Ä', 'ä', 'Â', 'â', 'À', 'à', 'Æ', 'æ', 'Ã', 'ã', 'E', 'e', 'É', 'é', 'Ë',
    'ë', 'Ê', 'ê', 'È', 'è', 'Œ', 'œ', 'I', 'i', 'Í', 'í', 'Ï', 'ï', 'Î', 'î', 'Ì', 'ì', 'O', 'o',
    'Ó', 'ó', 'Ö', 'ö', 'Ô', 'ô', 'Ò', 'ò', 'U', 'u', 'Ú', 'ú', 'Ü', 'ü', 'Û', 'û', 'Ù', 'ù', 'Y',
    'y',
];

/// Convert strings to Pig Latin. The first consonant of each word is moved to
/// the end of the word and ay is added, so first becomes irst-fay. Words that
/// start with a vowel have hay added to the end instead (apple becomes apple-
/// hay).
///
/// IMPORTANT: Only English letters are allowed (including letters from Old
/// English and letters with diacritics for borrowed words)!
fn convert_to_pig_latin(string: &str) -> Option<String> {
    if string.is_empty() {
        return Some(String::new());
    }

    for c in string.chars() {
        if !CONSONANTS.contains(&c) && !VOWELS.contains(&c) && !c.is_whitespace() {
            return None;
        }
    }

    let mut converted_words = Vec::new();

    for word in string.split_whitespace() {
        let mut char_iter = word.chars();

        if let Some(first_char) = char_iter.next() {
            if CONSONANTS.contains(&first_char) {
                if let Some(second_char) = char_iter.next() {
                    let mut converted_word = String::new();

                    if first_char.is_uppercase() {
                        converted_word.extend(second_char.to_uppercase());
                    } else {
                        converted_word.push(second_char)
                    }

                    converted_word.extend(char_iter);
                    converted_word.push('-');

                    if first_char.is_uppercase() {
                        converted_word.extend(first_char.to_lowercase());
                    } else {
                        converted_word.push(first_char)
                    }

                    converted_word.push('a');
                    converted_word.push('y');

                    converted_words.push(converted_word);
                } else {
                    converted_words.push(format!("{first_char}ay"));
                }
            } else if VOWELS.contains(&first_char) {
                converted_words.push(format!("{word}-hay"));
            }
        }
    }

    Some(converted_words.join(" "))
}

fn main() {
    let string = "Hello my friend Misha";
    let result = convert_to_pig_latin(string);

    match result {
        Some(val) => println!("{val}"),
        None => println!("Failed to convert \"{string}\" to Pig Latin, sorry!"),
    }
}
