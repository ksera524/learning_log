use super::chap1::word_score;

pub fn word_score_with_bonus(word: &String) -> i32 {
    let score = word_score(&word);
    if word.contains('c') {
        return score + 5;
    }
    score
}

pub fn word_score_with_bonus_and_penalty(word: &String) -> i32 {
    word_score(word) + bonus(word) + penalty(word)
}

pub fn bonus(word: &String) -> i32 {
    match word.contains('c') {
        true => 5,
        false => 0,
    }
}

pub fn penalty(word: &String) -> i32 {
    match word.contains('s') {
        true => -7,
        false => 0,
    }
}

pub fn ranked_word<F>(list: Vec<String>, f: F) -> Vec<String>
where
    F: Fn(&String) -> i32,
{
    let mut sorted_list: Vec<String> = list.into_iter().collect();
    sorted_list.sort_by_key(|b| std::cmp::Reverse(f(b)));
    sorted_list
}

pub fn word_scores<F>(list: Vec<String>, f: F) -> Vec<i32>
where
    F: Fn(&str) -> i32,
{
    list.into_iter().map(|s| f(&s)).collect()
}

pub fn high_scoreing_words<F>(list: Vec<String>, f: F) -> Vec<String>
where
    F: Fn(&str) -> i32,
{
    list.into_iter().filter(|s| f(s) > 1).collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranked_word_test() {
        let words = vec![
            "java".to_string(),
            "ada".to_string(),
            "haskell".to_string(),
            "scala".to_string(),
            "rust".to_string(),
        ];
        let sorted_words_bonus = ranked_word(words.clone(), word_score_with_bonus);
        let sorted_words_penalty = ranked_word(words.clone(), word_score_with_bonus_and_penalty);
        assert_eq!(
            vec![
                "scala".to_string(),
                "haskell".to_string(),
                "rust".to_string(),
                "java".to_string(),
                "ada".to_string()
            ],
            ranked_word(words, |word: &String| -> i32 {
                word_score(word) + bonus(word) - penalty(word)
            })
        );

        assert_eq!(
            vec![
                "scala".to_string(),
                "haskell".to_string(),
                "rust".to_string(),
                "java".to_string(),
                "ada".to_string()
            ],
            sorted_words_bonus
        );
        assert_eq!(
            vec![
                "java".to_string(),
                "ada".to_string(),
                "scala".to_string(),
                "haskell".to_string(),
                "rust".to_string()
            ],
            sorted_words_penalty
        );
    }

    #[test]
    fn word_scores_test() {
        let words = vec![
            "java".to_string(),
            "ada".to_string(),
            "haskell".to_string(),
            "scala".to_string(),
            "rust".to_string(),
        ];

        assert_eq!(vec![2, 1, 6, 3, 4], word_scores(words, word_score))
    }

    #[test]
    fn high_scoreing_words_test() {
        let words = vec![
            "java".to_string(),
            "ada".to_string(),
            "haskell".to_string(),
            "scala".to_string(),
            "rust".to_string(),
        ];

        assert_eq!(
            vec!["java".to_string()],
            high_scoreing_words(words, |word: &str| -> i32 {
                word_score(word) + bonus(&word.to_string()) + penalty(&word.to_string())
            })
        )
    }
}
