use std::sync::Arc;

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

pub fn high_scoreing_words<F>(
    list: Vec<String>
) -> Box<dyn Fn(F) -> Box<dyn Fn(i32) -> Vec<String>> >
where
    F: Fn(&str) -> i32  + 'static,
{
    let list = Arc::new(list);
    Box::new(move |f: F| {
        let list = Arc::clone(&list);
        Box::new(move |higher_than: i32| {
            list.iter()
                .filter(|word| f(word) > higher_than)
                .cloned()
                .collect()
        })
    })
}

pub fn larger_than(list:Vec<i32>) ->impl Fn(i32) -> Vec<i32> {
    move |higehr_than | {
        list.iter()
            .filter(|s| **s > higehr_than)
            .cloned()
            .collect()
    }
}

pub fn divisible_by(list:Vec<i32>) -> impl Fn(i32) -> Vec<i32> {
    move |divide| {
        list.iter()
            .filter(|s| **s % divide == 0)
            .cloned()
            .collect()
    }
}

pub fn shorter_than(list: Vec<String>) -> impl Fn(i32) -> Vec<String> {
    move |threshold| {
        list.iter()
            .filter(|word| (word.len() as i32) < threshold)
            .cloned()
            .collect()
    }
}

pub fn number_of_s(list:Vec<String>) -> impl Fn(i32) -> Vec<String> {
    move |threshold| {
        list.iter()
            .filter(|word| (word.len() - word.replace("s", "").len()) as i32 >= threshold)
            .cloned()
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use core::num;

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

        let score_fn = |word: &str| -> i32 {
            word_score(word) + bonus(&word.to_string()) + penalty(&word.to_string())
        };

        assert_eq!(
            vec!["java".to_string()],
            high_scoreing_words(words)(score_fn)(1)
        )
    }

    #[test]
    fn larger_than_test() {
        let list = vec![5,1,2,4,0];
        let func = larger_than(list);

        assert_eq!(vec![5],func(4));
        assert_eq!(vec![5,2,4],func(1));
    }

    #[test]
    fn divisible_by_test() {
        let list = vec!(5,1,2,4,15);

        assert_eq!(vec![5,15],divisible_by(list.clone())(5));
        assert_eq!(vec![2,4],divisible_by(list)(2));
    }

    #[test]
    fn shorter_than_test() {
        let list = vec!["scala".to_string(),"ada".to_string()];

        assert_eq!(vec!["scala".to_string(),"ada".to_string()],shorter_than(list.clone())(7));
        assert_eq!(vec!["ada".to_string()],shorter_than(list)(4));
    }

    #[test]
    fn number_of_s_test() {
        let list = vec!["rust".to_string(),"ada".to_string()];

        let expected: Vec<String> = vec![];
        assert_eq!(expected,number_of_s(list.clone())(3));
        assert_eq!(vec!["rust".to_string()],number_of_s(list)(1));
    }

    #[test]
    fn fold_test(){
        let list = vec![5,1,2,4,100];
        assert_eq!(112,list.iter().fold(0,|acc,&num| acc + num));

        let list = vec!["scala","rust","ada"];
        assert_eq!(12,list.iter().fold(0, |acc,word|acc + word.len()));

        let list = vec!["scala","rust","ada","haskell"];
        assert_eq!(3,list.iter().fold(0, |acc,word| acc + (word.len() - word.replace('s', "").len())));

        let list = vec![5,1,2,4,15];
        assert_eq!(15,list.iter().fold(0,|acc,&num| acc.max(num)));
    }
}
