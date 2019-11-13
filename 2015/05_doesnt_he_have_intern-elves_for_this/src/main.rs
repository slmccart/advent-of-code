use std::fs;

fn main() {
    let strings = read_input("input.txt");
    let strings: Vec<&str> = strings.split('\n').collect();
    let mut nice_strings = 0;

    for string in strings {
        if is_nice_string(string) { nice_strings += 1 }
    }

    println!("Found {} nice strings", nice_strings);
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
}

fn is_nice_string(s: &str) -> bool {
    contains_at_least_three_vowels(s) && contains_double_letters(s) && !contains_naughty_substring(s)
}

fn contains_at_least_three_vowels(s: &str) -> bool {
    let mut num_vowels_found = 0;

    for letter in s.chars() {
        match letter {
            'a' | 'e' | 'i' | 'o' | 'u' => num_vowels_found += 1,
            _ => (),
        }

        if num_vowels_found == 3 {
            return true;
        }
    }

    false
}

fn contains_double_letters(s: &str) -> bool {
    let mut last_letter = ' ';

    for letter in s.chars() {
        if letter == last_letter {
            return true;
        }

        last_letter = letter;
    }

    false
}

fn contains_naughty_substring(s: &str) -> bool {
    let disallowed_strings = ["ab", "cd", "pq", "xy"];

    for disallowed_string in disallowed_strings.iter() {
        if s.contains(disallowed_string) {
            return true;
        }
    }

    false
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
}