#[derive(Clone, Debug, PartialEq)]
struct TvShow {
    title: String,
    start: i32,
    end: i32,
}

impl TvShow {
    fn new(title: String, start: i32, end: i32) -> TvShow {
        TvShow { title, start, end }
    }
}
pub fn parse_shows(list: Vec<String>) -> Result<Vec<TvShow>, String> {
    list.iter()
        .fold(Ok(vec![]), |acc, show| add_or_resign(acc, parse_show(show)))
}

pub fn parse_show(show: &String) -> Result<TvShow, String> {
    Ok(TvShow::new(
        extract_name(show)?,
        extract_year_start(show).or_else(|_| extract_single_year(show))?,
        extract_year_end(show).or_else(|_| extract_single_year(show))?,
    ))
}

pub fn add_or_resign(
    parsed_shows: Result<Vec<TvShow>, String>,
    new_parsed_show: Result<TvShow, String>,
) -> Result<Vec<TvShow>, String> {
    parsed_shows.and_then(|mut shows| {
        new_parsed_show.map(|show| {
            shows.push(show);
            shows
        })
    })
}

pub fn extract_name(show: &str) -> Result<String, String> {
    let bracket_open = show
        .find('(')
        .ok_or(format!("Can't extract name from {}", show))?;
    Ok(show[..bracket_open].trim().to_string())
}

pub fn extract_year_start(show: &str) -> Result<i32, String> {
    let message = format!("Can't extract start year from {}", show);
    let bracket_open = show.find('(').ok_or(message.clone())?;
    let dash = show.find('-').ok_or(message.clone())?;
    show[bracket_open + 1..dash]
        .trim()
        .parse::<i32>()
        .map_err(|_| message)
}

pub fn extract_single_year(show: &str) -> Result<i32, String> {
    let message = format!("Can't extract from {}", show);
    let bracket_open = show.find('(').ok_or(message.clone())?;
    let bracket_close = show.find(')').ok_or(message.clone())?;

    if show[bracket_open + 1..bracket_close].contains('-') {
        return Err(message);
    }

    show[bracket_open + 1..bracket_close]
        .trim()
        .parse::<i32>()
        .map_err(|_| message)
}

pub fn extract_year_end(show: &str) -> Result<i32, String> {
    let message = format!("Can't extract start year from {}", show);
    let bracket_close = show.find(')').ok_or(message.clone())?;
    let dash = show.find('-').ok_or(message.clone())?;
    show[dash + 1..bracket_close]
        .trim()
        .parse::<i32>()
        .map_err(|_| message)
}

pub fn sort_show(list: Vec<TvShow>) -> Vec<TvShow> {
    let mut list = list.clone();
    list.sort_by_key(|tv| std::cmp::Reverse(tv.end - tv.start));

    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_show_test() {
        let list = vec![
            TvShow::new("Breaking Bad".to_string(), 2008, 2013),
            TvShow::new("The Wire".to_string(), 2002, 2008),
            TvShow::new("Mad men".to_string(), 2007, 2015),
        ];

        let expect = vec![
            TvShow::new("Mad men".to_string(), 2007, 2015),
            TvShow::new("The Wire".to_string(), 2002, 2008),
            TvShow::new("Breaking Bad".to_string(), 2008, 2013),
        ];

        assert_eq!(expect, sort_show(list));
    }

    #[test]
    fn parse_shows_test() {
        assert_eq!(
            Ok(vec![TvShow::new("Breaking Bad".to_string(), 2008, 2013)]),
            parse_shows(vec!["Breaking Bad (2008-2013)".to_string()])
        );
        assert_eq!(Ok(vec![]), parse_shows(vec![]));
        assert_eq!(
            Err("Can't extract from Chernobyl (-2019)".to_string()),
            parse_shows(vec![
                "Breaking Bad (2008-2013)".to_string(),
                "Chernobyl (-2019)".to_string()
            ])
        );

        assert_eq!(
            Err("Can't extract name from [2019]".to_string()),
            parse_shows(vec!["[2019]".to_string(),])
        );
    }
}
