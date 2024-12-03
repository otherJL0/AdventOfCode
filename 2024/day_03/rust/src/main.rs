use std::fs;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Token {
    Mul,
    Number(i32),
    LeftParen,
    RightParen,
    Comma,
    Invalid,
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
                if alpha == 'm'
                    && index + 2 < chars.len()
                    && chars[index + 1] == 'u'
                    && chars[index + 2] == 'l'
                {
                    tokens.push(Token::Mul);
                    index += 2;
                } else {
                    tokens.push(Token::Invalid);
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
            _ => tokens.push(Token::Invalid),
        }
        index += 1;
    }
    tokens
}

fn extract_mul_instructions(tokens: Vec<Token>) -> i32 {
    let mut result = 0;
    for window in tokens.windows(6) {
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
}

fn part_one(lines: Vec<String>) -> i32 {
    lines
        .into_iter()
        .map(|line| extract_mul_instructions(tokenize(line)))
        .sum()
}

fn main() {
    let part_one_result = part_one(read_input());
    println!("{part_one_result}");
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
}
