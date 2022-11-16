mod line_counter;

use line_counter::LineCounter;

/// All the types of tokens found in CSS
/// taken from https://www.w3.org/TR/css-syntax-3/#tokenization
#[derive(Debug, PartialEq)]
pub enum Token {
    BadComment(),
    Identifier(String),
    Function(String),
    AtKeyword(String),
    Hash(String, HashType),
    String(String),
    BadString(String),
    Url(String),
    BadUrl(String),
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

pub struct TokenAt {
    line: usize,
    column: usize,
    token: Token,
}

pub fn tokenize(input: &str) -> Vec<TokenAt> {
    let line_counter = LineCounter::new(input.chars());

    let mut tokens = Vec::new();

    for (line, column, character) in line_counter {
        let token = match character {
            '/' => todo!("add support for comments"),
            ' ' | '\t' | '\r' | '\n' => todo!("add support for whitespace"),
            'a'..='z' | 'A'..='Z' | '_' => {
                todo!("add support for identifiers, functions, and urls")
            }
            '0'..='9' => todo!("add support for numbers, percentages, and dimensions"),
            '#' => todo!("add support for hashes"),
            '"' => todo!("add support for strings"),
            '\'' => todo!("add support for strings"),
            '+' => todo!("add support for numbers, percentages, and dimensions"),
            '-' => todo!("add support for numbers, percentages, and dimensions and identifiers"),
            '.' => todo!("add support for numbers, percentages, and dimensions"),
            '@' => todo!("add support for at keywords"),
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

        tokens.push(TokenAt {
            line,
            column,
            token,
        });
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lambda() {
        let input = "";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn comment_empty() {
        let input = "/**/";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn comment_with_text() {
        let input = "/* comment */";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn comment_that_does_not_end() {
        let input = "/* comment";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::BadComment());
    }

    #[test]
    fn identifier_with_one_character() {
        let input = "a";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::Identifier("a".to_string()));
    }

    #[test]
    fn identifier_with_multiple_characters() {
        let input = "abc";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::Identifier("abc".to_string()));
    }

    #[test]
    fn identifier_with_underscore() {
        let input = "a_bc";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::Identifier("a_bc".to_string()));
    }

    #[test]
    fn identifier_with_hyphen() {
        let input = "a-bc";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::Identifier("a-bc".to_string()));
    }

    #[test]
    fn identifier_with_leading_underscore() {
        let input = "_abc";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::Identifier("_abc".to_string()));
    }

    #[test]
    fn identifier_with_leading_hyphen() {
        let input = "-abc";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::Identifier("-abc".to_string()));
    }

    #[test]
    fn function_with_one_character() {
        let input = "a(";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::Function("a".to_string()));
    }

    #[test]
    fn function_with_multiple_characters() {
        let input = "abc(";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::Function("abc".to_string()));
    }

    #[test]
    fn at_media() {
        let input = "@media";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::AtKeyword("media".to_string()));
    }

    #[test]
    fn at_import() {
        let input = "@import";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::AtKeyword("import".to_string()));
    }

    #[test]
    fn hash_delimiter() {
        let input = "#";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::Delimiter('#'));
    }

    #[test]
    fn hash_one_letter() {
        let input = "#a";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens[0].token,
            Token::Hash("a".to_string(), HashType::Unrestricted)
        );
    }

    #[test]
    fn hash_two_letters() {
        let input = "#a";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens[0].token,
            Token::Hash("a".to_string(), HashType::Unrestricted)
        );
    }

    #[test]
    fn hash_three_letters() {
        let input = "#abc";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens[0].token,
            Token::Hash("abc".to_string(), HashType::Id)
        );
    }

    #[test]
    fn hash_four_letters() {
        let input = "#abcd";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens[0].token,
            Token::Hash("abcd".to_string(), HashType::Id)
        );
    }

    #[test]
    fn hash_one_digit() {
        let input = "#1";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens[0].token,
            Token::Hash("1".to_string(), HashType::Unrestricted)
        );
    }

    #[test]
    fn hash_four_digits() {
        let input = "#1234";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens[0].token,
            Token::Hash("1234".to_string(), HashType::Unrestricted)
        );
    }

    #[test]
    fn hash_unrestricted_one_character() {
        let input = "#a";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens[0].token,
            Token::Hash("a".to_string(), HashType::Unrestricted)
        );
    }

    #[test]
    fn string_empty_double() {
        let input = "\"\"";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::String("".to_string()));
    }

    #[test]
    fn string_empty_single() {
        let input = "''";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::String("".to_string()));
    }

    #[test]
    fn string_one_character_double() {
        let input = "\"a\"";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::String("a".to_string()));
    }

    #[test]
    fn string_one_character_single() {
        let input = "'a'";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::String("a".to_string()));
    }

    #[test]
    fn string_many_characters_double() {
        let input = "\"abc def\"";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::String("abc def".to_string()));
    }

    #[test]
    fn string_many_characters_single() {
        let input = "'abc def'";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::String("abc def".to_string()));
    }

    #[test]
    fn string_with_special_characters() {
        let input = "\"!@#$%^&*-+=;:,.?/`~|()[]{}\"";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens[0].token,
            Token::String("!@#$%^&*-+=;:,.?/`~|()[]{}".to_string())
        );
    }

    #[test]
    fn string_with_escaped_double() {
        let input = "\"\\\"\"";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::String("\"".to_string()));
    }

    #[test]
    fn string_with_escaped_single() {
        let input = "'\\''";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::String("'".to_string()));
    }

    #[test]
    fn string_with_escaped_newline() {
        let input = "\"\\\n\"";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::String("\n".to_string()));
    }

    #[test]
    fn string_with_no_close() {
        let input = "\"abc";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::BadString("abc".to_string()));
    }

    #[test]
    fn string_interrupted_by_newline() {
        let input = "\"abc\n";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::BadString("abc".to_string()));
    }

    #[test]
    fn todo_simple() {
        todo!("add tests for untested things");
    }

    #[test]
    fn whitespace_one_space() {
        let input = " ";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::Whitespace());
    }

    #[test]
    fn whitespace_many_spaces() {
        let input = "   ";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::Whitespace());
    }

    #[test]
    fn whitespace_one_tab() {
        let input = "\t";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::Whitespace());
    }

    #[test]
    fn whitespace_newline() {
        let input = "\n";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::Whitespace());
    }

    #[test]
    fn whitespace_carriage_return() {
        let input = "\r";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::Whitespace());
    }

    #[test]
    fn whitespace_many_characters() {
        let input = "\t\t\t        \r\n   \r\n  \n \r\r\t";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::Whitespace());
    }

    #[test]
    fn colon() {
        let input = ":";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::Colon());
    }

    #[test]
    fn semicolon() {
        let input = ";";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::Semicolon());
    }

    #[test]
    fn comma() {
        let input = ",";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::Comma());
    }

    #[test]
    fn parenthesis() {
        let input = "()";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token, Token::OpenParenthesis());
        assert_eq!(tokens[1].token, Token::CloseParenthesis());
    }

    #[test]
    fn square_brackets() {
        let input = "[]";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token, Token::OpenSquareBracket());
        assert_eq!(tokens[1].token, Token::CloseSquareBracket());
    }

    #[test]
    fn curly_brackets() {
        let input = "{}";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token, Token::OpenCurlyBracket());
        assert_eq!(tokens[1].token, Token::CloseCurlyBracket());
    }

    #[test]
    fn todo_complex() {
        todo!("add test for actual CSS code");
    }
}
