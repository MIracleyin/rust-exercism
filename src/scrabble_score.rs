use std::collections::{HashMap, HashSet};

/// a hash record CHAR: score
/// iter word and add their score

/// Compute the Scrabble score for a word.
fn dictionary() -> HashMap<char, u64> {
    hashmap! {
        'a' => 1,
        'b' => 3,
        'c' => 3,
        'd' => 2,
        'e' => 1,
        'f' => 4,
        'g' => 2,
        'h' => 4,
        'i' => 1,
        'j' => 8,
        'k' => 5,
        'l' => 1,
        'm' => 3,
        'n' => 1,
        'o' => 1,
        'p' => 3,
        'q' => 10,
        'r' => 1,
        's' => 1,
        't' => 1,
        'u' => 1,
        'v' => 4,
        'w' => 4,
        'x' => 8,
        'y' => 4,
        'z' => 10,
    }
}
pub fn score(word: &str) -> u64 {
    let dic = dictionary();
    word.to_lowercase()
        .chars()
        .map(|c| dic.get(&c))
        .fold(0, |score, v| score + v.unwrap_or(&0))
}

#[test]
fn a_is_worth_one_point() {
    assert_eq!(score("a"), 1);
}

#[test]
//#[ignore]
fn scoring_is_case_insensitive() {
    assert_eq!(score("A"), 1);
}

#[test]
//#[ignore]
fn f_is_worth_four() {
    assert_eq!(score("f"), 4);
}

#[test]
//#[ignore]
fn two_one_point_letters_make_a_two_point_word() {
    assert_eq!(score("at"), 2);
}

#[test]
//#[ignore]
fn three_letter_word() {
    assert_eq!(score("zoo"), 12);
}

#[test]
//#[ignore]
fn medium_word() {
    assert_eq!(score("street"), 6);
}

#[test]
//#[ignore]
fn longer_words_with_valuable_letters() {
    assert_eq!(score("quirky"), 22);
}

#[test]
//#[ignore]
fn long_mixed_case_word() {
    assert_eq!(score("OxyphenButazone"), 41);
}

#[test]
//#[ignore]
fn non_english_scrabble_letters_do_not_score() {
    assert_eq!(score("pinata"), 8, "'n' should score 1");
    assert_eq!(score("piñata"), 7, "'ñ' should score 0");
}

#[test]
//#[ignore]
fn empty_words_are_worth_zero() {
    assert_eq!(score(""), 0);
}

#[test]
//#[ignore]
fn all_letters_work() {
    assert_eq!(score("abcdefghijklmnopqrstuvwxyz"), 87);
}

#[test]
//#[ignore]
fn german_letters_do_not_score() {
    assert_eq!(score("STRASSE"), 7, "\"SS\" should score 2");
    assert_eq!(score("STRAßE"), 5, "'ß' should score 0");
}
