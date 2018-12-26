use regex::Regex;
use std::borrow::Cow;
use std::str::Chars;
use unicode_normalization::char::{canonical_combining_class, decompose_compatible};
use unicode_normalization::UnicodeNormalization;
use unidecode::unidecode_char;

const HYPHENS: &str = "-\u{2010}‑‒–—―−－﹘﹣";
const ASCII_UNUSUAL_WHITESPACE: &[char] = &['\t', '\r', '\n'];

pub fn is_mixed_case(s: &str) -> bool {
    let mut has_lowercase = false;
    let mut has_uppercase = false;

    for c in s.chars() {
        if c.is_uppercase() {
            has_uppercase = true;
        };
        if c.is_lowercase() {
            has_lowercase = true;
        };
        if has_lowercase && has_uppercase {
            return true;
        };
    }

    false
}

#[inline]
pub fn is_combining(c: char) -> bool {
    canonical_combining_class(c) > 0
}

#[inline]
pub fn is_ascii_alphabetic(c: char) -> bool {
    match c {
        'a'...'z' => true,
        'A'...'Z' => true,
        _ => false,
    }
}

// Sadly necessary because string split gives "type of this value must be known"
// compilation error when passed a closure in some contexts
#[inline]
pub fn is_nonalphanumeric(c: char) -> bool {
    !c.is_alphanumeric()
}

#[inline]
pub fn lowercase_if_alpha(c: char) -> Option<char> {
    if c.is_uppercase() {
        c.to_lowercase().next()
    } else if c.is_alphabetic() {
        Some(c)
    } else {
        None
    }
}

#[inline]
pub fn uppercase_if_alpha(c: char) -> Option<char> {
    if c.is_lowercase() {
        c.to_uppercase().next()
    } else if c.is_alphabetic() {
        Some(c)
    } else {
        None
    }
}

#[inline]
pub fn transliterate(c: char) -> Chars<'static> {
    unidecode_char(c).chars()
}

#[inline]
pub fn to_ascii_letter(c: char) -> Option<char> {
    debug_assert!(c.is_uppercase(), c.to_string());
    match c {
        'A'...'Z' => Some(c),
        _ => match transliterate(c).next() {
            Some(c) => c.to_uppercase().next(),
            None => None,
        },
    }
}

pub fn to_ascii(s: &str) -> Cow<str> {
    if s.is_ascii() {
        Cow::Borrowed(s)
    } else {
        let mut capitalized_any = false;

        Cow::Owned(
            s.chars()
                .flat_map(transliterate)
                .filter_map(|c| {
                    if !c.is_alphabetic() {
                        None
                    } else if c.is_uppercase() && !capitalized_any {
                        capitalized_any = true;
                        Some(c)
                    } else if c.is_lowercase() && capitalized_any {
                        Some(c)
                    } else {
                        c.to_lowercase().next()
                    }
                })
                .collect(),
        )
    }
}

pub fn capitalize_word(word: &str, simple: bool) -> String {
    debug_assert!(simple == word.chars().all(is_ascii_alphabetic));

    if simple {
        let bytes = word.as_bytes();
        let mut result = String::with_capacity(word.len());
        result.push(bytes[0].to_ascii_uppercase() as char);
        result.extend(bytes[1..].iter().map(|c| c.to_ascii_lowercase() as char));
        result
    } else {
        let mut capitalize_next = true;

        word.chars()
            .map(|c| {
                let result = if capitalize_next {
                    c.to_uppercase().next()
                } else {
                    c.to_lowercase().next()
                }
                .unwrap();

                // If the character doesn't have both uppercase and lowercase versions,
                // it'll be unchanged. That's a prerequisite for it being a separator.
                capitalize_next = result == c && !c.is_alphanumeric() && !is_combining(c);

                result
            })
            .collect()
    }
}

// Ideally we'd use the unicode standard `quick_check` algorithm, but Rust
// doesn't expose that for compatibility forms, and this is simpler to implement,
// though it will have more false negatives.
fn stable_nfkd(c: char) -> bool {
    if is_combining(c) {
        false
    } else {
        let mut stable = true;
        decompose_compatible(c, |d| stable = stable && d == c);
        stable
    }
}

pub fn normalize_nfkd_hyphens_spaces(string: &str) -> Cow<str> {
    if string.is_ascii() && !string.contains(ASCII_UNUSUAL_WHITESPACE) {
        Cow::Borrowed(string)
    } else if string
        .chars()
        .all(|c| stable_nfkd(c) && (c == ' ' || !c.is_whitespace()))
    {
        Cow::Borrowed(string)
    } else {
        let string = string
            .nfkd()
            .map(|c| {
                if HYPHENS.contains(c) {
                    '-'
                } else if c.is_whitespace() {
                    ' '
                } else {
                    c
                }
            })
            .collect();

        Cow::Owned(string)
    }
}

#[derive(Debug, Clone)]
pub struct CharacterCounts {
    pub chars: u8,
    pub alpha: u8,
    pub upper: u8,
    pub ascii_alpha: u8,
    pub ascii_vowels: u8,
}

pub fn categorize_chars(word: &str) -> CharacterCounts {
    debug_assert!(word.len() <= u8::max_value() as usize);

    let mut chars = 0;
    let mut alpha = 0;
    let mut upper = 0;
    let mut ascii_alpha = 0;
    let mut ascii_vowels = 0;

    for c in word.chars() {
        match c {
            'a'...'z' => {
                if "aeiouy".contains(c) {
                    ascii_vowels += 1;
                } else {
                    ascii_alpha += 1;
                }
            }
            'A'...'Z' => {
                if "AEIOUY".contains(c) {
                    ascii_vowels += 1;
                } else {
                    ascii_alpha += 1;
                }
                upper += 1;
            }
            _ if c.is_uppercase() => {
                alpha += 1;
                upper += 1;
            }
            _ if c.is_alphabetic() => {
                alpha += 1;
            }
            _ => {
                chars += 1;
            }
        }
    }

    // Maybe skipping individual increments and doing this instead is
    // premature optimization, but why not
    ascii_alpha += ascii_vowels;
    alpha += ascii_alpha;
    chars += alpha;

    CharacterCounts {
        chars,
        alpha,
        upper,
        ascii_alpha,
        ascii_vowels,
    }
}

pub fn starts_with_consonant(word: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[a-zA-Z--aeiouAEIOU]").unwrap();
    }
    RE.is_match(word)
}

#[inline]
pub fn starts_with_uppercase(word: &str) -> bool {
    word.chars().nth(0).filter(|c| c.is_uppercase()).is_some()
}

pub fn combining_chars(word: &str) -> usize {
    word.chars().filter(|c| is_combining(*c)).count()
}

#[inline]
pub fn has_number(word: &str) -> bool {
    word.chars().any(char::is_numeric)
}

pub fn has_sequential_alphas(word: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\p{Alphabetic}{2,}").unwrap();
    }
    RE.is_match(word)
}

#[macro_export]
macro_rules! eq_or_starts_with {
    ($a:expr, $b:expr) => {{
        let mut chars_a = $a.chars().filter_map(lowercase_if_alpha);
        let mut chars_b = $b.chars().filter_map(lowercase_if_alpha);
        let result;

        loop {
            let a = chars_a.next();
            let b = chars_b.next();

            if a.is_none() || b.is_none() {
                result = true;
                break;
            } else if a != b {
                result = false;
                break;
            }
        }

        result
    }};
}

#[macro_export]
macro_rules! eq_or_ends_with {
    ($needle:expr, $haystack:expr) => {{
        let mut n_chars = $needle.chars().rev().filter_map(lowercase_if_alpha);
        let mut h_chars = $haystack.chars().rev().filter_map(lowercase_if_alpha);
        let result;

        loop {
            let n = n_chars.next();
            let h = h_chars.next();

            if n.is_none() {
                result = true;
                break;
            } else if n != h {
                result = false;
                break;
            }
        }

        result
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn sequential_alphas() {
        assert!(has_sequential_alphas("ab"));
        assert!(has_sequential_alphas("abc"));
        assert!(has_sequential_alphas("a.bc"));
        assert!(has_sequential_alphas("鄭a"));
        assert!(!has_sequential_alphas(""));
        assert!(!has_sequential_alphas("a"));
        assert!(!has_sequential_alphas("a.b"));
        assert!(!has_sequential_alphas("鄭.a"));
        assert!(!has_sequential_alphas("ﾟ."));
    }

    #[test]
    fn capitalization() {
        assert_eq!("A", capitalize_word("a", true));
        assert_eq!("Aa", capitalize_word("aa", true));
        assert_eq!("Aa", capitalize_word("AA", true));
    }

    #[bench]
    fn is_mixed_case_false(b: &mut Bencher) {
        b.iter(|| black_box(is_mixed_case("JOHN MACDONALD")))
    }

    #[bench]
    fn is_mixed_case_true(b: &mut Bencher) {
        b.iter(|| black_box(is_mixed_case("J. MacDonald")))
    }

    #[bench]
    fn capitalize_uppercase_word(b: &mut Bencher) {
        b.iter(|| black_box(capitalize_word("JONATHAN", true)))
    }

    #[bench]
    fn capitalize_complex_word(b: &mut Bencher) {
        b.iter(|| black_box(capitalize_word("föö-bar", false)))
    }

    #[bench]
    fn normalize_ascii(b: &mut Bencher) {
        b.iter(|| black_box(normalize_nfkd_hyphens_spaces("James 'J' S. Brown MD").len()))
    }

    #[bench]
    fn normalize_nfkd_stable(b: &mut Bencher) {
        b.iter(|| black_box(normalize_nfkd_hyphens_spaces("James «J» S. Brown MD").len()))
    }

    #[bench]
    fn normalize_needs_fix(b: &mut Bencher) {
        b.iter(|| black_box(normalize_nfkd_hyphens_spaces("James 'J' S. Bröwn MD").len()))
    }
}
