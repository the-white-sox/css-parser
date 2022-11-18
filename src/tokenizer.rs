use std::iter::Peekable;

mod line_counter;

#[cfg(test)]
mod tests;

use line_counter::LineCounter;

/// All the types of tokens found in CSS
///
/// adapted from https://www.w3.org/TR/css-syntax-3/#tokenization
#[derive(Debug, PartialEq)]
pub enum Token {
    BadComment(),
    Identifier(String),
    Function(String),
    AtKeyword(String),
    Hash(String, HashType),
    String(String),
    BadString(),
    Url(String),
    BadUrl(),
    Delimiter(char),
    Number(f64),
    Percentage(f64),
    Dimension(f64, String),
    Whitespace(),
    Colon(),
    Semicolon(),
    Comma(),
    OpenSquareBracket(),
    CloseSquareBracket(),
    OpenParenthesis(),
    CloseParenthesis(),
    OpenCurlyBracket(),
    CloseCurlyBracket(),
}

#[derive(Debug, PartialEq, Eq)]
pub enum HashType {
    Id,
    Unrestricted,
}

#[derive(Debug)]
pub struct TokenAt {
    pub line: usize,
    pub column: usize,
    pub token: Token,
}

/// Converts a iterator of characters into an iterator of tokens
pub struct Tokenizer<I: Iterator<Item = char>> {
    chars: Peekable<LineCounter<I>>,
}

impl<I: Iterator<Item = char>> Tokenizer<I> {
    /// Creates a new tokenizer from a iterator of characters
    pub fn new(chars: I) -> Self {
        Self {
            chars: LineCounter::new(chars).peekable(),
        }
    }

    /// consumes whitespace from chars
    fn consume_whitespace(&mut self) {
        while let Some((_, _, character)) = self.chars.peek() {
            match character {
                ' ' | '\t' | '\r' | '\n' => {
                    self.chars.next();
                }
                _ => break,
            }
        }
    }

    /// If the next character would start a identifier
    fn would_start_identifier(&mut self) -> bool {
        match self.chars.peek() {
            Some((_, _, character)) => match character {
                'a'..='z' | 'A'..='Z' | '_' => true,
                '\\' => todo!("add support for escapes"),
                _ => false,
            },
            None => false,
        }
    }

    /// Consumes a sequence of numbers letters hyphens and underscores
    fn consume_identifier_sequence(&mut self) -> String {
        let mut identifier = String::new();

        while let Some(&(_, _, character)) = self.chars.peek() {
            match character {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => {
                    self.chars.next();
                    identifier.push(character);
                }
                '\\' => {
                    todo!("add support for escapes");
                }
                _ => break,
            }
        }

        identifier
    }

    /// Consumes a sequence of numbers letters hyphens and underscores then returns a Token
    ///
    /// can return a Token::Identifier, Token::Function, Token::Url, or Token::BadUrl
    fn consume_identifier_like_token(&mut self, first_character: char) -> Token {
        let mut identifier = self.consume_identifier_sequence();

        identifier.insert(0, first_character);

        if let Some((_, _, '(')) = self.chars.peek() {
            self.chars.next();

            if identifier.eq_ignore_ascii_case("url") {
                self.consume_whitespace();

                if let Some((_, _, '"' | '\'')) = self.chars.peek() {
                    Token::Function(identifier)
                } else {
                    self.consume_url_token()
                }
            } else {
                Token::Function(identifier)
            }
        } else {
            Token::Identifier(identifier)
        }
    }

    /// Consumes a url token
    ///
    /// Assumes that `url(` has already been consumed
    ///
    /// returns a Token::Url or Token::BadUrl
    fn consume_url_token(&mut self) -> Token {
        self.consume_whitespace();

        let mut url = String::new();

        while let Some(&(_, _, character)) = self.chars.peek() {
            match character {
                ')' => break,
                ' ' | '\t' | '\r' | '\n' => break,
                '"' | '\'' | '(' | '\0' | '\x08' | '\x0B' | '\x0E'..='\x1F' | '\x7F' => break,
                '\\' => {
                    todo!("add support for escapes");
                }
                _ => {
                    self.chars.next();
                    url.push(character);
                }
            }
        }

        self.consume_whitespace();

        if let Some((_, _, ')')) = self.chars.peek() {
            self.chars.next();
            Token::Url(url)
        } else {
            Token::BadUrl()
        }
    }

    /// Consume a number
    fn consume_number(&mut self, first_character: char) -> f64 {
        let mut number = String::new();

        number.push(first_character);

        let mut has_dot = first_character == '.';

        while let Some(&(_, _, character)) = self.chars.peek() {
            match character {
                '0'..='9' => {
                    self.chars.next();
                    number.push(character);
                }
                '.' => {
                    if has_dot {
                        break;
                    }
                    // technically "5." is not a valid number in CSS but we will allow it
                    self.chars.next();
                    number.push(character);
                    has_dot = true;
                }
                _ => break,
            }
        }

        number
            .parse()
            .expect("failed to parse number, this should never happen")
    }

    /// Consume a numeric token
    ///
    /// can return a Token::Number, Token::Percentage, or Token::Dimension
    fn consume_numeric_token(&mut self, first_character: char) -> Token {
        let number = self.consume_number(first_character);

        match self.chars.peek() {
            Some((_, _, 'a'..='z' | 'A'..='Z' | '_')) => {
                let unit = self.consume_identifier_sequence();
                Token::Dimension(number, unit)
            }
            Some((_, _, '%')) => {
                self.chars.next();
                Token::Percentage(number)
            }
            _ => Token::Number(number),
        }
    }

    /// Consumes a string token
    fn consume_string_token(&mut self, end_character: char) -> Token {
        let mut string = String::new();

        #[allow(clippy::while_let_on_iterator)] // because we are not exhausting chars
        while let Some((_, _, character)) = self.chars.next() {
            match character {
                '"' | '\'' => {
                    if character == end_character {
                        return Token::String(string);
                    } else {
                        string.push(character);
                    }
                }
                '\n' => {
                    return Token::BadString();
                }
                '\\' => match self.chars.next() {
                    Some((_, _, character)) => match character {
                        '\r' => {
                            if let Some((_, _, '\n')) = self.chars.peek() {
                                self.next();
                            }
                        }
                        '\n' => {}
                        _ => {
                            string.push(character);
                        }
                    },
                    None => {
                        return Token::BadString();
                    }
                },
                _ => {
                    string.push(character);
                }
            }
        }

        Token::BadString()
    }
}

impl<I: Iterator<Item = char>> Iterator for Tokenizer<I> {
    type Item = TokenAt;

    fn next(&mut self) -> Option<Self::Item> {
        let (line, column, character) = self.chars.next()?;

        // adapted from https://www.w3.org/TR/css-syntax-3/#consume-token
        let token = match character {
            // comments
            '/' => {
                if let Some((_, _, '*')) = self.chars.peek() {
                    self.chars.next();

                    while let Some((_, _, character)) = self.chars.next() {
                        if character == '*' {
                            if let Some((_, _, '/')) = self.chars.peek() {
                                self.chars.next();
                                return self.next();
                            }
                        }
                    }

                    Token::BadComment()
                } else {
                    Token::Delimiter('/')
                }
            }

            // whitespace
            ' ' | '\t' | '\r' | '\n' => {
                self.consume_whitespace();
                Token::Whitespace()
            }

            // identifiers, functions, and urls
            'a'..='z' | 'A'..='Z' | '_' => self.consume_identifier_like_token(character),

            // ids and hashes
            '#' => match self.chars.peek() {
                Some((_, _, next_character)) => match next_character {
                    'a'..='z' | 'A'..='Z' | '_' => {
                        Token::Hash(self.consume_identifier_sequence(), HashType::Id)
                    }
                    '0'..='9' | '-' => {
                        Token::Hash(self.consume_identifier_sequence(), HashType::Unrestricted)
                    }
                    '\\' => todo!("add support for escapes"),
                    _ => Token::Delimiter('#'),
                },
                None => Token::Delimiter('#'),
            },

            // strings
            '"' => self.consume_string_token('"'),
            '\'' => self.consume_string_token('\''),

            // numbers
            '0'..='9' => self.consume_numeric_token(character),
            '+' => match self.chars.peek() {
                Some(&(_, _, next_character)) => match next_character {
                    '0'..='9' | '.' => self.consume_numeric_token(character),
                    _ => Token::Delimiter('+'),
                },
                None => Token::Delimiter('+'),
            },
            '-' => match self.chars.peek() {
                Some(&(_, _, next_character)) => match next_character {
                    '0'..='9' | '.' => self.consume_numeric_token(character),
                    'a'..='z' | 'A'..='Z' | '_' | '-' => {
                        self.consume_identifier_like_token(character)
                    }
                    _ => Token::Delimiter('-'),
                },
                None => Token::Delimiter('-'),
            },
            '.' => match self.chars.peek() {
                Some((_, _, '0'..='9')) => self.consume_numeric_token(character),
                _ => Token::Delimiter('.'),
            },

            // at keywords
            '@' => {
                if self.would_start_identifier() {
                    Token::AtKeyword(self.consume_identifier_sequence())
                } else {
                    Token::Delimiter('@')
                }
            }

            '\\' => todo!("add support for escapes"),
            ':' => Token::Colon(),
            ';' => Token::Semicolon(),
            ',' => Token::Comma(),
            '(' => Token::OpenParenthesis(),
            ')' => Token::CloseParenthesis(),
            '[' => Token::OpenSquareBracket(),
            ']' => Token::CloseSquareBracket(),
            '{' => Token::OpenCurlyBracket(),
            '}' => Token::CloseCurlyBracket(),
            _ => Token::Delimiter(character),
        };

        Some(TokenAt {
            line,
            column,
            token,
        })
    }
}
