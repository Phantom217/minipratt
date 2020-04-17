use std::{fmt, io::BufRead};

enum S {
    Cons(char, Vec<S>),
}

impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S::Cons(head, rest) => {
                if rest.is_empty() {
                    write!(f, "{}", head)
                } else {
                    write!(f, "({}", head)?;
                    for s in rest {
                        write!(f, " {}", s)?
                    }
                    write!(f, ")")
                }
            }
        }
    }
}

struct Lexer {
    tokens: Vec<char>,
}

impl Lexer {
    fn new(input: &str) -> Lexer {
        let mut tokens = input
            .chars()
            .filter(|it| !it.is_ascii_whitespace())
            .collect::<Vec<_>>();
        tokens.reverse();
        Lexer { tokens }
    }

    fn next(&mut self) -> Option<char> {
        self.tokens.pop()
    }

    fn peek(&mut self) -> Option<char> {
        self.tokens.last().copied()
    }
}

fn expr(input: &str) -> S {
    let mut lexer = Lexer::new(input);
    expr_bp(&mut lexer, 0).unwrap()
}

fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> Option<S> {
    let mut lhs = None;

    loop {
        let token = match lexer.peek() {
            Some(token) => token,
            None => return lhs,
        };

        let r_bp = match binding_power(token, lhs.is_none()) {
            Some((l_bp, r_bp)) if min_bp <= l_bp => r_bp,
            _ => return lhs,
        };
        lexer.next();

        let rhs = expr_bp(lexer, r_bp);
        if token == '(' {
            assert_eq!(lexer.next(), Some(')'));
            lhs = rhs;
            continue;
        }

        let mut args = Vec::new();
        args.extend(lhs);
        args.extend(rhs);
        lhs = Some(S::Cons(token, args));
    }
}

/// Compute left/right binding power for an operator.
fn binding_power(op: char, prefix: bool) -> Option<(u8, u8)> {
    let res = match op {
        '0'..='9' | 'a'..='z' | 'A'..='Z' => (99, 100),
        '(' => (99, 0),
        '=' => (2, 1),
        '+' | '-' if prefix => (99, 9),
        '+' | '-' => (5, 6),
        '*' | '/' => (7, 8),
        '!' => (11, 100),
        '.' => (14, 13),
        _ => return None,
    };
    Some(res)
}

fn main() {
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let s = expr(&line);
        println!("{}", s);
    }
}
