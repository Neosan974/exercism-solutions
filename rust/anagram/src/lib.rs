use std::collections::HashSet;
use itertools::Itertools;
use unicode_segmentation::UnicodeSegmentation;

fn parse_string(input: &str) -> String {
    input.graphemes(true).collect::<String>().to_lowercase()
}

fn order_chars(input: String) -> String {
    input.chars().sorted().join("")
}

fn is_anagram_of(possible_anagram: &str, word: &str) -> bool {
    if possible_anagram.len() != word.len() {
        return false
    }
    let a = parse_string(possible_anagram);
    let b = parse_string(word);
    if a == b {
        return false
    }
    let word_ordered_letters = order_chars(a);
    let possible_anagram_ordered_letters = order_chars(b);
    word_ordered_letters == possible_anagram_ordered_letters
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    for &possible_anagram in possible_anagrams {
        if is_anagram_of(possible_anagram, word) {
            anagrams.insert(possible_anagram);
        }
    }
    anagrams
}
