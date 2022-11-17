use std::iter::Peekable;

mod line_counter;

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

#[derive(Debug, PartialEq)]
pub enum HashType {
    Id,
    Unrestricted,
}

#[derive(Debug)]
pub struct TokenAt {
    line: usize,
    column: usize,
    token: Token,
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
                todo!("add support for urls");
            } else {
                Token::Function(identifier)
            }
        } else {
            Token::Identifier(identifier)
        }
    }

    /// Consumes a string token
    fn consume_string_token(&mut self, end_character: char) -> Token {
        let mut string = String::new();

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
                '\\' => {
                    todo!("add support for escapes");
                }
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

            '0'..='9' => todo!("add support for numbers, percentages, and dimensions"),

            // ids and hashes
            '#' => match self.chars.peek() {
                Some((_, _, character)) => match character {
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

            '+' => todo!("add support for numbers, percentages, and dimensions"),
            '-' => todo!("add support for numbers, percentages, and dimensions and identifiers"),
            '.' => todo!("add support for numbers, percentages, and dimensions"),

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

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_tokens(input: &str, expected: Vec<Token>) {
        let mut tokens = Tokenizer::new(input.chars()).map(|token_at| token_at.token);

        let mut expected_tokens = expected.into_iter();

        loop {
            let token = tokens.next();
            let expected_token = expected_tokens.next();

            if token.is_none() && expected_token.is_none() {
                break;
            }

            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn lambda() {
        assert_tokens("", vec![]);
    }

    #[test]
    fn comment_empty() {
        assert_tokens("/**/", vec![]);
    }

    #[test]
    fn comment_with_text() {
        assert_tokens("/* comment */", vec![]);
    }

    #[test]
    fn comment_that_does_not_end() {
        assert_tokens("/* comment", vec![Token::BadComment()]);
    }

    #[test]
    fn comment_delimiter() {
        assert_tokens("/", vec![Token::Delimiter('/')]);
    }

    #[test]
    fn identifier_with_one_character() {
        assert_tokens("a", vec![Token::Identifier("a".to_string())]);
    }

    #[test]
    fn identifier_with_multiple_characters() {
        assert_tokens("abc", vec![Token::Identifier("abc".to_string())]);
    }

    #[test]
    fn identifier_with_underscore() {
        assert_tokens("a_bc", vec![Token::Identifier("a_bc".to_string())]);
    }

    #[test]
    fn identifier_with_hyphen() {
        assert_tokens("a-bc", vec![Token::Identifier("a-bc".to_string())]);
    }

    #[test]
    fn identifier_with_leading_underscore() {
        assert_tokens("_abc", vec![Token::Identifier("_abc".to_string())]);
    }

    #[test]
    fn identifier_with_leading_hyphen() {
        assert_tokens("-abc", vec![Token::Identifier("-abc".to_string())]);
    }

    #[test]
    fn function_with_one_character() {
        assert_tokens("a(", vec![Token::Function("a".to_string())]);
    }

    #[test]
    fn function_with_multiple_characters() {
        assert_tokens("abc(", vec![Token::Function("abc".to_string())]);
    }

    #[test]
    fn function_with_hyphen_and_underscore() {
        assert_tokens("a-b_c(", vec![Token::Function("a-b_c".to_string())]);
    }

    #[test]
    fn function_url_double_quote() {
        assert_tokens(
            "url(\"https://example.com/image.png\")",
            vec![
                Token::Function("url".to_string()),
                Token::String("https://example.com/image.png".to_string()),
                Token::CloseParenthesis(),
            ],
        );
    }

    #[test]
    fn function_url_single_quote() {
        assert_tokens(
            "url('https://example.com/image.png')",
            vec![
                Token::Function("url".to_string()),
                Token::String("https://example.com/image.png".to_string()),
                Token::CloseParenthesis(),
            ],
        );
    }

    #[test]
    fn url_empty() {
        assert_tokens("url()", vec![Token::Url("".to_string())]);
    }

    #[test]
    fn url_one_character() {
        assert_tokens("url(a)", vec![Token::Url("a".to_string())]);
    }

    #[test]
    fn url_example_image() {
        assert_tokens(
            "url(https://example.com/image.png)",
            vec![Token::Url("https://example.com/image.png".to_string())],
        );
    }

    #[test]
    fn url_with_whitespace() {
        assert_tokens(
            "url(   https://example.com/image.png   )",
            vec![Token::Url("https://example.com/image.png".to_string())],
        );
    }

    #[test]
    fn url_upper_case() {
        assert_tokens(
            "URL(https://example.com/image.png)",
            vec![Token::Url("https://example.com/image.png".to_string())],
        );
    }

    #[test]
    fn url_mixed_case() {
        assert_tokens(
            "uRL(https://example.com/image.png)",
            vec![Token::Url("https://example.com/image.png".to_string())],
        );
    }

    #[test]
    fn bad_url_interrupted_by_whitespace() {
        assert_tokens("url(https://url.with spaces.com)", vec![Token::BadUrl()]);
    }

    #[test]
    fn bad_url_with_double_quote() {
        assert_tokens("url(https://url.with\"quote.com)", vec![Token::BadUrl()]);
    }

    #[test]
    fn bad_url_with_single_quote() {
        assert_tokens("url(https://url.with'quote.com)", vec![Token::BadUrl()]);
    }

    #[test]
    fn bad_url_with_open_paren() {
        assert_tokens("url(https://url.with(paren.com)", vec![Token::BadUrl()]);
    }

    #[test]
    fn bad_url_with_null() {
        assert_tokens("url(https://url.with\0null.com)", vec![Token::BadUrl()]);
    }

    #[test]
    fn at_media() {
        assert_tokens("@media", vec![Token::AtKeyword("media".to_string())]);
    }

    #[test]
    fn at_import() {
        assert_tokens("@import", vec![Token::AtKeyword("import".to_string())]);
    }

    #[test]
    fn at_delimiter() {
        assert_tokens("@", vec![Token::Delimiter('@')]);
    }

    #[test]
    fn hash_delimiter() {
        assert_tokens("#", vec![Token::Delimiter('#')]);
    }

    #[test]
    fn hash_one_letter() {
        assert_tokens("#a", vec![Token::Hash("a".to_string(), HashType::Id)]);
    }

    #[test]
    fn hash_three_letters() {
        assert_tokens("#abc", vec![Token::Hash("abc".to_string(), HashType::Id)]);
    }

    #[test]
    fn hash_four_letters() {
        assert_tokens("#abcd", vec![Token::Hash("abcd".to_string(), HashType::Id)]);
    }

    #[test]
    fn hash_one_digit() {
        assert_tokens(
            "#1",
            vec![Token::Hash("1".to_string(), HashType::Unrestricted)],
        );
    }

    #[test]
    fn hash_four_digits() {
        assert_tokens(
            "#1234",
            vec![Token::Hash("1234".to_string(), HashType::Unrestricted)],
        );
    }

    #[test]
    fn hash_followed_by_whitespace() {
        assert_tokens(
            "#abc ",
            vec![
                Token::Hash("abc".to_string(), HashType::Id),
                Token::Whitespace(),
            ],
        );
    }

    #[test]
    fn hash_followed_by_colon() {
        assert_tokens(
            "#abc:",
            vec![Token::Hash("abc".to_string(), HashType::Id), Token::Colon()],
        );
    }

    #[test]
    fn string_empty_double() {
        assert_tokens("\"\"", vec![Token::String("".to_string())]);
    }

    #[test]
    fn string_empty_single() {
        assert_tokens("''", vec![Token::String("".to_string())]);
    }

    #[test]
    fn string_one_character_double() {
        assert_tokens("\"a\"", vec![Token::String("a".to_string())]);
    }

    #[test]
    fn string_one_character_single() {
        assert_tokens("'a'", vec![Token::String("a".to_string())]);
    }

    #[test]
    fn string_many_characters_double() {
        assert_tokens("\"abc def\"", vec![Token::String("abc def".to_string())]);
    }

    #[test]
    fn string_many_characters_single() {
        assert_tokens("'abc def'", vec![Token::String("abc def".to_string())]);
    }

    #[test]
    fn string_with_special_characters() {
        assert_tokens(
            "\"!@#$%^&*-+=;:,.?/`~|()[]{}\"",
            vec![Token::String("!@#$%^&*-+=;:,.?/`~|()[]{}".to_string())],
        );
    }

    #[test]
    fn string_with_escaped_double() {
        assert_tokens("\"\\\"\"", vec![Token::String("\"".to_string())]);
    }

    #[test]
    fn string_with_escaped_single() {
        assert_tokens("'\\''", vec![Token::String("'".to_string())]);
    }

    #[test]
    fn string_with_escaped_newline() {
        assert_tokens("\"\\\n\"", vec![Token::String("\n".to_string())]);
    }

    #[test]
    fn string_with_no_close() {
        assert_tokens("\"abc", vec![Token::BadString()]);
    }

    #[test]
    fn string_interrupted_by_newline() {
        assert_tokens("\"abc\n", vec![Token::BadString()]);
    }

    #[test]
    fn number_zero() {
        assert_tokens("0", vec![Token::Number(0.0)]);
    }

    #[test]
    fn number_one() {
        assert_tokens("1", vec![Token::Number(1.0)]);
    }

    #[test]
    fn number_negative_one() {
        assert_tokens("-1", vec![Token::Number(-1.0)]);
    }

    #[test]
    fn number_positive_one() {
        assert_tokens("+1", vec![Token::Number(1.0)]);
    }

    #[test]
    fn number_one_point_zero() {
        assert_tokens("1.0", vec![Token::Number(1.0)]);
    }

    #[test]
    fn number_one_point_five() {
        assert_tokens("1.5", vec![Token::Number(1.5)]);
    }

    #[test]
    fn number_point_five() {
        assert_tokens(".5", vec![Token::Number(0.5)]);
    }

    #[test]
    fn percentage_zero() {
        assert_tokens("0%", vec![Token::Percentage(0.0)]);
    }

    #[test]
    fn percentage_one() {
        assert_tokens("1%", vec![Token::Percentage(1.0)]);
    }

    #[test]
    fn percentage_one_hundred() {
        assert_tokens("100%", vec![Token::Percentage(100.0)]);
    }

    #[test]
    fn percentage_negative_three_hundred() {
        assert_tokens("-300%", vec![Token::Percentage(-300.0)]);
    }

    #[test]
    fn percentage_positive_point_five() {
        assert_tokens("+.5%", vec![Token::Percentage(0.5)]);
    }

    #[test]
    fn dimension_zero_px() {
        assert_tokens("0px", vec![Token::Dimension(0.0, "px".to_string())]);
    }

    #[test]
    fn dimension_negative_three_em() {
        assert_tokens("-3em", vec![Token::Dimension(-3.0, "em".to_string())]);
    }

    #[test]
    fn whitespace_one_space() {
        assert_tokens(" ", vec![Token::Whitespace()]);
    }

    #[test]
    fn whitespace_many_spaces() {
        assert_tokens("   ", vec![Token::Whitespace()]);
    }

    #[test]
    fn whitespace_one_tab() {
        assert_tokens("\t", vec![Token::Whitespace()]);
    }

    #[test]
    fn whitespace_newline() {
        assert_tokens("\n", vec![Token::Whitespace()]);
    }

    #[test]
    fn whitespace_carriage_return() {
        assert_tokens("\r", vec![Token::Whitespace()]);
    }

    #[test]
    fn whitespace_many_characters() {
        assert_tokens(
            "\t\t\t        \r\n   \r\n  \n \r\r\t",
            vec![Token::Whitespace()],
        );
    }

    #[test]
    fn colon() {
        assert_tokens(":", vec![Token::Colon()]);
    }

    #[test]
    fn semicolon() {
        assert_tokens(";", vec![Token::Semicolon()]);
    }

    #[test]
    fn comma() {
        assert_tokens(",", vec![Token::Comma()]);
    }

    #[test]
    fn parenthesis() {
        assert_tokens(
            "()",
            vec![Token::OpenParenthesis(), Token::CloseParenthesis()],
        );
    }

    #[test]
    fn square_brackets() {
        assert_tokens(
            "[]",
            vec![Token::OpenSquareBracket(), Token::CloseSquareBracket()],
        );
    }

    #[test]
    fn curly_brackets() {
        assert_tokens(
            "{}",
            vec![Token::OpenCurlyBracket(), Token::CloseCurlyBracket()],
        );
    }

    #[test]
    fn delimiters() {
        assert_tokens(
            "<>*~🐈",
            vec![
                Token::Delimiter('<'),
                Token::Delimiter('>'),
                Token::Delimiter('*'),
                Token::Delimiter('~'),
                Token::Delimiter('🐈'),
            ],
        );
    }

    #[test]
    fn todo_complex() {
        todo!("add test for actual CSS code");
    }
}
