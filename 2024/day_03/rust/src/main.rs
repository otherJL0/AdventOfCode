use std::fs;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Token {
    Mul,
    Number(u32),
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
        match &chars[index..] {
            ['(', _rest @ ..] => {
                tokens.push(Token::LeftParen);
                index += 1;
            }
            [')', _rest @ ..] => {
                tokens.push(Token::RightParen);
                index += 1;
            }
            [',', _rest @ ..] => {
                tokens.push(Token::Comma);
                index += 1;
            }
            ['m', 'u', 'l', _rest @ ..] => {
                tokens.push(Token::Mul);
                index += 3;
            }
            ['d', 'o', '(', ')', _rest @ ..] => {
                tokens.push(Token::Do);
                index += 4;
            }
            ['d', 'o', 'n', '\'', 't', '(', ')', _rest @ ..] => {
                tokens.push(Token::Dont);
                index += 7;
            }
            [a, b, c, _rest @ ..]
                if a.is_ascii_digit() && b.is_ascii_digit() && c.is_ascii_digit() =>
            {
                tokens.push(Token::Number(
                    a.to_digit(10).unwrap() * 100
                        + b.to_digit(10).unwrap() * 10
                        + c.to_digit(10).unwrap(),
                ));
                index += 3;
            }
            [a, b, _rest @ ..] if a.is_ascii_digit() && b.is_ascii_digit() => {
                tokens.push(Token::Number(
                    a.to_digit(10).unwrap() * 10 + b.to_digit(10).unwrap(),
                ));
                index += 2;
            }

            [a, _rest @ ..] if a.is_ascii_digit() => {
                tokens.push(Token::Number(a.to_digit(10).unwrap()));
                index += 1;
            }
            [other, _rest @ ..] => {
                tokens.push(Token::Invalid(*other));
                index += 1;
            }
            [] => break,
        }
    }
    tokens
}

fn part_one(tokens: &[Token]) -> u32 {
    tokens.windows(6).filter_map(|window| match window {
        [Token::Mul, Token::LeftParen, Token::Number(a), Token::Comma, Token::Number(b), Token::RightParen] => Some(a * b),
        _ => None,
    }).sum()
}

fn part_two(tokens: &[Token]) -> u32 {
    let mut result: u32 = 0;
    let mut will_add = true;

    tokens.windows(6).for_each(|window| {
        match window {
            [Token::Do, ..] => will_add = true,
            [Token::Dont, ..] => will_add = false,
            [Token::Mul, Token::LeftParen, Token::Number(a), Token::Comma, Token::Number(b), Token::RightParen] => if will_add {result += a * b},
            _ => {},
        }
    });
    result
}

fn main() {
    let input = read_input();
    let tokens = tokenize(input.into_iter().collect());

    let part_one_result = part_one(&tokens);
    println!("{part_one_result}");

    let part_two_result = part_two(&tokens);
    println!("{part_two_result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _expr() {
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
