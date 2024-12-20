#[derive(Debug, PartialEq, Eq)]
enum Token {
    Number,
    Ident
}

fn main() {
    let input = "123 world";
    println!("source: {:?}, parsed: {:?}", input, source(input));

    let input = "Hello world";
    println!("source: {:?}, parsed: {:?}", input, source(input));

    let input = "      world";
    println!("source: {:?}, parsed: {:?}", input, source(input));
}

fn recognizer(input: &str) -> &str {
    input
}

fn whitespace(mut input: &str) -> &str {
    while matches!(input.chars().next(), Some(' ')) {
        input = advance_char(input);
    }
    input
}

fn number(mut input: &str) -> (&str, Option<Token>) {
    if matches!(peek_char(input), Some(_x @ ('-' | '+' | '.' | '0'..='9'))) {
        
        while matches!(peek_char(input), Some(_x @ ('.' | '0'..='9'))) {
            input = advance_char(input);
        }
        (input, Some(Token::Number))
    } else {
        (input, None)
    }
}

fn ident(mut input: &str) -> (&str, Option<Token>) {
    if matches!(peek_char(input), Some(_x @ ('_' | 'a'..='z' | 'A'..='Z'))) {
        while matches!(
            peek_char(input),
            Some(_x @ ('a'..='z' | 'A'..='Z' | '0'..='9'))
        ) {
            input = advance_char(input);
        }
        (input, Some(Token::Ident))
    } else {
        (input, None)
    }
}

fn token(i: &str) -> (&str, Option<Token>) {
    if let (i, Some(ident_res)) = ident(whitespace(i)) {
        return (i, Some(ident_res));
    }
    if let (i, Some(number_res)) = number(whitespace(i)) {
        return (i, Some(number_res));
    }
    (i, None)
}

fn source(mut input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    while !input.is_empty() {
        input = if let (next_input, Some(token)) = token(input) {
            tokens.push(token);
            next_input
        } else {
            break;
        };
    }
    tokens
}

fn advance_char(input: &str) -> &str {
    let mut chars = input.chars();
    chars.next();
    chars.as_str()
}

fn peek_char(input: &str) -> Option<char> {
    input.chars().next()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_whitespace() {
        assert_eq!(whitespace("123 world"), "123 world");
    }

    #[test]
    fn test_ident() {
        assert_eq!(("", Some(Token::Ident)), ident("abc"));
        assert_eq!(("!test", Some(Token::Ident)), ident("hello123!test"));
    }

    #[test]
    fn test_number() {
        assert_eq!(("", Some(Token::Number)), number("123"));
        assert_eq!(("", Some(Token::Number)), number("123.456"));
    }
}
