use std::fs;
use pcre::Pcre;

fn main() {
    let strings = read_input("input.txt");
    let strings: Vec<&str> = strings.split('\n').collect();
    let mut nice_strings = 0;
    let mut nice_strings_new_rules = 0;

    for string in strings {
        if is_nice_string(string) { nice_strings += 1 }
        if is_nice_string_by_new_rules(string) { nice_strings_new_rules += 1 }
    }

    println!("Found {} nice strings", nice_strings);
    println!("Found {} nice strings by the new rules", nice_strings_new_rules);
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
}

fn is_nice_string(s: &str) -> bool {
    contains_at_least_three_vowels(s) && contains_double_letters(s) && !contains_naughty_substring(s)
}

fn is_nice_string_by_new_rules(s: &str) -> bool {
    contains_two_independent_pairs(s) && contains_letter_sandwich(s)
}

fn contains_at_least_three_vowels(s: &str) -> bool {
    // Any number of characters followed by any vowel - 3 or more times
    let mut re = Pcre::compile(r"(.*[aeiou]){3,}").unwrap();
    re.exec(s).is_some()
}

fn contains_double_letters(s: &str) -> bool {
    // Any character - repeated once
    let mut re = Pcre::compile(r"(.)\1").unwrap();
    re.exec(s).is_some()
}

fn contains_naughty_substring(s: &str) -> bool {
    let mut re = Pcre::compile(r"ab|cd|pq|xy").unwrap();
    re.exec(s).is_some()
}

fn contains_two_independent_pairs(s: &str) -> bool {
    // Any two characters, followed by any number of characters, then repeat the first group
    let mut re = Pcre::compile(r"(..).*\1").unwrap();
    re.exec(s).is_some()
}

fn contains_letter_sandwich(s: &str) -> bool {
    // Any character, followed by any character, then repeat the first group
    let mut re = Pcre::compile(r"(.).\1").unwrap();
    re.exec(s).is_some()

    //**** Alternative answer using rolling windows */

    //Get an iterator of rolling Windows containing exactly 3 characters each
    // abcdefeghi becomes abc bcd cde def efe ...
    // let characters = s.chars().collect::<Vec<char>>();
    // let windows = characters.windows(3);

    // for window in windows {
    //     //If the window contains the same character in the 1st and 3rd positions, then this string is a letter sandwich
    //     if window[0] == window[2] { return true; }
    // }

    // false
}

#[cfg(test)]
mod tests {

    #[test]
    fn contains_naughty_substring() {
        assert_eq!(super::contains_naughty_substring("abcdpqxy"), true);
        assert_eq!(super::contains_naughty_substring("aaaaaaaa"), false);
        assert_eq!(super::contains_naughty_substring("aacdaaaa"), true);
    }

    #[test]
    fn contains_at_least_three_vowels() {
        assert_eq!(super::contains_at_least_three_vowels("aei"), true);
        assert_eq!(super::contains_at_least_three_vowels("aaa"), true);
        assert_eq!(super::contains_at_least_three_vowels("xazegov"), true);
        assert_eq!(super::contains_at_least_three_vowels("aeiouaeiouaeiou"), true);
        assert_eq!(super::contains_at_least_three_vowels("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn contains_double_letters() {
        assert_eq!(super::contains_double_letters("aa"), true);
        assert_eq!(super::contains_double_letters("aaa"), true);
        assert_eq!(super::contains_double_letters("ugknbfddgicrmopn"), true);
        assert_eq!(super::contains_double_letters("jchzalrnumimnmhp"), false);
    }

    #[test]
    fn is_nice_string() {
        assert_eq!(super::is_nice_string("ugknbfddgicrmopn"), true);
        assert_eq!(super::is_nice_string("aaa"), true);
        assert_eq!(super::is_nice_string("jchzalrnumimnmhp"), false);
        assert_eq!(super::is_nice_string("haegwjzuvuyypxyu"), false);
        assert_eq!(super::is_nice_string("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn contains_two_independent_pairs() {
        assert_eq!(super::contains_two_independent_pairs("xyxy"), true);
        assert_eq!(super::contains_two_independent_pairs("xxyxx"), true);
        assert_eq!(super::contains_two_independent_pairs("aabcdefgaa"), true);
        assert_eq!(super::contains_two_independent_pairs("aaa"), false);
        assert_eq!(super::contains_two_independent_pairs("bcaaacb"), false);
    }

    #[test]
    fn contains_letter_sandwich() {
        assert_eq!(super::contains_letter_sandwich("xyx"), true);
        assert_eq!(super::contains_letter_sandwich("abcdefeghi"), true);
        assert_eq!(super::contains_letter_sandwich("aaa"), true);
        assert_eq!(super::contains_letter_sandwich("abc"), false);
    }

    #[test]
    fn is_nice_string_by_new_rules() {
        assert_eq!(super::is_nice_string_by_new_rules("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(super::is_nice_string_by_new_rules("xxyxx"), true);
        assert_eq!(super::is_nice_string_by_new_rules("uurcxstgmygtbstg"), false);
        assert_eq!(super::is_nice_string_by_new_rules("ieodomkazucvgmuy"), false);
    }
}