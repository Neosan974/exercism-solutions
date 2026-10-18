use std::collections::HashSet;
use itertools::Itertools;
use unicode_segmentation::UnicodeSegmentation;

fn is_anagram_of(possible_anagram: &str, word: &str) -> bool {
    if possible_anagram.len() != word.len() {
        return false
    }
    let a: String = possible_anagram.graphemes(true).collect();
    let b: String = word.graphemes(true).collect();
    if a.to_lowercase() == b.to_lowercase() {
        return false
    }
    let word_ordered_letters = a.to_lowercase().chars().sorted().join("");
    let possible_anagram_ordered_letters = b.to_lowercase().chars().sorted().join("");
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
