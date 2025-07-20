// Token struct that includes column number for enhanced error messages
#[derive(Debug)]
pub struct TokenSpan {
    pub token: Token,
    pub span: usize,
}

#[derive(Debug)]
pub enum Token {
    LParen,
    RParen,
    Const(u32),
    Operator(char),
    UnIdent(char),
}

pub fn tokenize(src: String) -> Vec<TokenSpan> {
    let mut output = Vec::<TokenSpan>::new();
    let mut remaining: Vec<char> = src.chars().collect();
    let mut index: usize = 0;

    while !remaining.is_empty() {
        let mut front = remaining.remove(0);

        let mut token_span: Option<TokenSpan> = None;

        // match single character tokens
        match front {
            '(' => {
                token_span = Some(TokenSpan {
                    token: Token::LParen,
                    span: index,
                })
            }
            ')' => {
                token_span = Some(TokenSpan {
                    token: Token::RParen,
                    span: index,
                })
            }
            '+' | '*' => {
                token_span = Some(TokenSpan {
                    token: Token::Operator(front),
                    span: index,
                });
            }
            _ => { /* do nothing and let number parser take a shot */ }
        }

        // parse const numbers, they are likely to take up more than one char
        let mut num = String::from("");
        while front.is_digit(10) {
            num.push(front);

            // if no more chars to parse, or the next char is not a number
            if remaining.is_empty() || !remaining[0].is_digit(10) {
                break;
            }
            front = remaining.remove(0);
        }

        // create the const number token span
        if num != "" {
            token_span = Some(TokenSpan {
                token: Token::Const(
                    num.parse::<u32>()
                        .expect("should be built out of acceptable digits"),
                ),
                span: index,
            });
            index += num.len() - 1;
        }

        if front != ' ' || token_span.is_some() {
            output.push(token_span.unwrap_or(TokenSpan {
                token: Token::UnIdent(front),
                span: index,
            }));
        }

        index += 1;
    }

    output
}
