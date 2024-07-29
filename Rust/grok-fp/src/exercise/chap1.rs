pub fn increment(x: i32) -> i32 {
    x + 1
}

pub fn get_first_char(s: &str) -> char {
    s.chars().next().unwrap()
}

pub fn word_score(word: &str) -> i32 {
    word.replace('a', "").len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        assert_eq!(increment(1), 2);
    }

    #[test]
    fn test_get_first_char() {
        assert_eq!(get_first_char("hello"), 'h');
    }

    #[test]
    fn test_word_score() {
        assert_eq!(word_score("helloaa"), 5);
    }
}
