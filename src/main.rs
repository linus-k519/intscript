use std::io;
use std::io::Write;
use std::str::Chars;

struct Lexer {
    text: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(text: Vec<char>) -> Lexer {
        Lexer {
            text,
            pos: 0,
        }
    }

    fn parse(&mut self) {
        while self.pos < self.text.len() {
            let current_char = self.text[self.pos];
            match current_char {
                c if c.is_ascii_digit() => println!("Parsed number: {}", self.make_number()),
                _ => println!("Parsed token: {:?}", self.make_token()),
            }
        }
    }

    fn make_number(&mut self) -> i64 {
        let mut number_str = String::new();
        while self.pos < self.text.len() {
            let current_char = self.text[self.pos];

            if !(current_char.is_ascii_digit() || current_char == '_') {
                // End parsing at non-digit char (_ excluded)
                break;
            }
            self.pos += 1;
            number_str.push(current_char);
        }

        // Parse string to number
        number_str.parse::<i64>().unwrap()
    }

    fn make_token(&mut self) -> Token {
        let current_char = self.text[self.pos];
        let return_value = match current_char {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Mul,
            '/' => Token::Div,
            c => panic!("Invalid char {}", c),
        };
        self.pos += 1;
        return_value
    }
}

#[derive(Debug)]
enum Token {
    Plus, Minus, Mul, Div
}

fn main() {
    // Read input
    let mut input = String::new();
    print!("Input: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    Lexer::new(input.chars().filter(|c| !c.is_whitespace()).collect()).parse();
}
