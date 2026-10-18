use std::collections::HashSet;
use itertools::Itertools;

fn is_anagram_of(possible_anagram: &str, word: &str) -> bool {
    if possible_anagram.len() != word.len() {
        return false
    }
    if possible_anagram.to_lowercase() == word.to_lowercase() {
        return false
    }
    let word_ordered_letters = word.to_lowercase().chars().sorted().join("");
    let possible_anagram_ordered_letters = word.to_lowercase().chars().sorted().join("");
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
