pub fn abbreviate(name: &str) -> String {
    let mut parts = name.split_whitespace();
    let first = parts.next().unwrap();
    let last = parts.next().unwrap();
    format!("{}. {}", first.chars().next().unwrap(), last)
}

pub fn first_two(list:&Vec<String>) -> Vec<String> {
    list[0..2].to_vec()
}

pub fn last_two(list:&Vec<String>) -> Vec<String> {
    list[list.len()-2..list.len()].to_vec()
}

pub fn move_first_to_the_end(list:&Vec<String>) -> Vec<String> {
    let first = &list[0..2];
    vec!(list[2..].to_vec(), first.to_vec()).concat()
}

pub fn insert_before_last(list:&Vec<String>, item:String) -> Vec<String> {
    let mut new_list = list.clone();
    new_list.insert(list.len()-1, item);
    new_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abbreviate() {
        assert_eq!(abbreviate("John Doe"), "J. Doe");
    }

    #[test]
    fn test_first_two() {
        let list = vec!["John".to_string(), "Doe".to_string(), "Jane".to_string()];
        assert_eq!(first_two(&list), vec!["John".to_string(), "Doe".to_string()]);
    }

    #[test]
    fn test_last_two() {
        let list = vec!["John".to_string(), "Doe".to_string(), "Jane".to_string()];
        assert_eq!(last_two(&list), vec!["Doe".to_string(), "Jane".to_string()]);
    }

    #[test]
    fn test_move_first_to_the_end() {
        let list = vec!["John".to_string(), "Doe".to_string(), "Jane".to_string()];
        assert_eq!(move_first_to_the_end(&list), vec!["Jane".to_string(), "John".to_string(), "Doe".to_string()]);
    }

    #[test]
    fn test_insert_before_last() {
        let list = vec!["John".to_string(), "Doe".to_string(), "Jane".to_string()];
        assert_eq!(insert_before_last(&list, "Smith".to_string()), vec!["John".to_string(),  "Doe".to_string(), "Smith".to_string(),"Jane".to_string()]);
    }
}