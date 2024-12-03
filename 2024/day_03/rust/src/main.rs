use std::fs;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Token {
    Mul,
    Number(i32),
    LeftParen,
    RightParen,
    Comma,
    Invalid(char),
    Do,
    Dont,
}

fn read_input() -> Vec<String> {
    fs::read_to_string("../input")
        .expect("Failed to read input")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().to_string())
        .collect()
}

fn tokenize(line: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut index: usize = 0;
    let chars: Vec<char> = line.chars().collect();
    while index < chars.len() {
        let c = chars[index];
        match c {
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            ',' => tokens.push(Token::Comma),
            alpha if alpha.is_ascii_alphabetic() => {
                let is_mul = chars[index..=index + 2].iter().collect::<String>() == "mul";
                let is_do = chars[index..=index + 3].iter().collect::<String>() == "do()";
                let is_dont = chars[index..=index + 6].iter().collect::<String>() == "don't()";
                if is_mul {
                    tokens.push(Token::Mul);
                    index += 2;
                } else if is_do {
                    tokens.push(Token::Do);
                    index += 3;
                } else if is_dont {
                    tokens.push(Token::Dont);
                    index += 6;
                } else {
                    tokens.push(Token::Invalid(chars[index]));
                }
            }
            digit if digit.is_ascii_digit() => {
                let start = index;
                let mut end = index;
                while end < chars.len() && chars[end].is_ascii_digit() {
                    end += 1;
                }
                let value = chars[start..end]
                    .iter()
                    .collect::<String>()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
                tokens.push(Token::Number(value));
                index = end - 1;
            }
            _ => tokens.push(Token::Invalid(chars[index])),
        }
        index += 1;
    }
    tokens
}

fn part_one(lines: Vec<String>) -> i32 {
    lines
        .into_iter()
        .map(|line| {
            let mut result = 0;
            for window in tokenize(line).windows(6) {
                if window[0] == Token::Mul
                    && window[1] == Token::LeftParen
                    && window[3] == Token::Comma
                    && window[5] == Token::RightParen
                {
                    match (window[2], window[4]) {
                        (Token::Number(a), Token::Number(b)) => result += a * b,
                        _ => continue,
                    }
                }
            }
            result
        })
        .sum()
}

fn part_two(lines: Vec<String>) -> i32 {
    lines
        .into_iter()
        .map(|line| {
            let mut result: i32 = 0;
            let mut will_add = true;
            let tokens = tokenize(line);
            let mut index: usize = 0;
            while index < tokens.len() {
                match &tokens[index..] {
                    [Token::Do, _rest @ ..] =>  will_add = true,
                    [Token::Dont, _rest @ ..] =>  will_add = false,
                    [Token::Mul, Token::LeftParen, Token::Number(a), Token::Comma, Token::Number(b), Token::RightParen, _rest @ ..] => {
                        if will_add {
                            result += a * b;
                        }
                        index += 5;
                    }
                    [_other, _rest @ ..] => (),
                    [] => break,
                }
                index += 1;
            }
            result
        })
        .sum()
}

fn main() {
    let part_one_result = part_one(read_input());
    println!("{part_one_result}");
    let part_two_result = part_two(read_input());
    println!("{part_two_result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_expr() {
        let input = String::from("mul(1,10)");
        let expected = vec![
            Token::Mul,
            Token::LeftParen,
            Token::Number(1),
            Token::Comma,
            Token::Number(10),
            Token::RightParen,
        ];
        let actual = tokenize(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn valid_expr_with_do() {
        let input = String::from("do()mul(1,10)");
        let expected = vec![
            Token::Do,
            Token::Mul,
            Token::LeftParen,
            Token::Number(1),
            Token::Comma,
            Token::Number(10),
            Token::RightParen,
        ];
        let actual = tokenize(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn valid_expr_with_dont() {
        let input = String::from("don't()mul(1,10)");
        let expected = vec![
            Token::Dont,
            Token::Mul,
            Token::LeftParen,
            Token::Number(1),
            Token::Comma,
            Token::Number(10),
            Token::RightParen,
        ];
        let actual = tokenize(input);
        assert_eq!(expected, actual);
    }
}
