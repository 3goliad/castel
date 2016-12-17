use std::collections::VecDeque;
use tokens::*;

pub fn lex<I>(chars: &mut I) -> VecDeque<Token>
    where I: Iterator<Item = char>
{
    let mut lexemes: VecDeque<Token> = VecDeque::<Token>::new();
    let mut tok_buffer: TokenBuffer = TokenBuffer::new();
    let mut quote = false;

    for c in chars {
        if tok_buffer.match_operator().is_some() {
            if tok_buffer.matches_operator_plus(c) {
                tok_buffer.push(c);
                continue;
            } else {
                lexemes.push_back(tok_buffer.dump());
                tok_buffer.push(c);
                continue;
            }
        } else if (c == '\'') | (c == '"') | (c == '\\') {
            tok_buffer.push(c);
            if tok_buffer.last_char() != '\\' {
                if quote {
                    quote = false;
                } else {
                    quote = true;
                }
            }
            continue;
        } else if c == '\n' {
            lexemes.push_back(Token::Newline);
        }

    }

    if !tok_buffer.is_empty() {
        lexemes.push_back(tok_buffer.dump());
    }

    lexemes.push_back(Token::EOF);

    lexemes
}
