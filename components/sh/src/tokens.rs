#[derive(PartialEq)]
pub enum Token {
    Token(String),
    EOF,
    Newline,
    IO_NUMBER,
    ASSIGNMENT_WORD,
    NAME,
    Operator(Operator),
    ReservedWord(ReservedWord),
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub enum Operator {
    AND_IF, // &&
    OR_IF, // ||
    DSEMI, // ;;
    DLESS, // <<
    DGREAT, // >>
    LESSAND, // <&
    GREATAND, // >&
    LESSGREAT, // <>
    DLESSDASH, // <<-
    CLOBBER, // >|
}

#[derive(PartialEq)]
pub enum ReservedWord {
    If,
    Then,
    Else,
    Elif,
    Fi,
    Do,
    Done,
    Case,
    Esac,
    While,
    Until,
    For,
    Lbrace, // {
    Rbrace, // }
    Bang, // !
    In,
}

pub struct TokenBuffer {
    buf: String,
}

impl TokenBuffer {
    pub fn new() -> TokenBuffer {
        TokenBuffer { buf: String::new() }
    }

    pub fn push(&mut self, c: char) {
        self.buf.push(c);
    }

    pub fn match_operator(&self) -> Option<Token> {
        use tokens::Operator::*;

        Some(Token::Operator(
            match self.buf.as_str() {
            "&&" => AND_IF,
            "||" => OR_IF,
            ";;" => DSEMI,
            "<<" => DLESS,
            ">>" => DGREAT,
            "<&" => LESSAND,
            ">&" => GREATAND,
            "<>" => LESSGREAT,
            "<<-" => DLESSDASH,
            ">|" => CLOBBER,
            _ => {return None},
        }))
    }

    pub fn matches_operator_plus(&mut self, c: char) -> bool {
        self.buf.push(c);
        let o = self.match_operator();
        self.buf.pop();
        o.is_some()
    }

    pub fn dump(&mut self) -> Token {
        let tok = self.match_operator().unwrap();
        self.buf = String::new();
        return tok;
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    pub fn last_char(&self) -> char {
        match self.buf.chars().last() {
            None => '\x00',
            Some(c) => c,
        }
    }
}
