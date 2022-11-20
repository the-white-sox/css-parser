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

mod comments {
    use super::*;

    #[test]
    fn empty() {
        assert_tokens("/**/", vec![]);
    }

    #[test]
    fn with_text() {
        assert_tokens("/* comment */", vec![]);
    }

    #[test]
    fn that_does_not_end() {
        assert_tokens("/* comment", vec![Token::BadComment()]);
    }

    #[test]
    fn delimiter() {
        assert_tokens("/", vec![Token::Delimiter('/')]);
    }
}

mod identifiers {
    use super::*;

    #[test]
    fn one_character() {
        assert_tokens("a", vec![Token::Identifier("a".to_owned())]);
    }

    #[test]
    fn multiple_characters() {
        assert_tokens("abc", vec![Token::Identifier("abc".to_owned())]);
    }

    #[test]
    fn underscore() {
        assert_tokens("a_bc", vec![Token::Identifier("a_bc".to_owned())]);
    }

    #[test]
    fn hyphen() {
        assert_tokens("a-bc", vec![Token::Identifier("a-bc".to_owned())]);
    }

    #[test]
    fn leading_underscore() {
        assert_tokens("_abc", vec![Token::Identifier("_abc".to_owned())]);
    }

    #[test]
    fn leading_hyphen() {
        assert_tokens("-abc", vec![Token::Identifier("-abc".to_owned())]);
    }

    #[test]
    fn leading_double_hyphen() {
        assert_tokens("--abc", vec![Token::Identifier("--abc".to_owned())]);
    }
}

mod functions {
    use super::*;

    #[test]
    fn one_character() {
        assert_tokens("a(", vec![Token::Function("a".to_owned())]);
    }

    #[test]
    fn multiple_characters() {
        assert_tokens("abc(", vec![Token::Function("abc".to_owned())]);
    }

    #[test]
    fn hyphen_and_underscore() {
        assert_tokens("a-b_c(", vec![Token::Function("a-b_c".to_owned())]);
    }

    #[test]
    fn url_double_quote() {
        assert_tokens(
            "url(\"https://example.com/image.png\")",
            vec![
                Token::Function("url".to_owned()),
                Token::String("https://example.com/image.png".to_owned()),
                Token::CloseParenthesis(),
            ],
        );
    }

    #[test]
    fn url_single_quote() {
        assert_tokens(
            "url('https://example.com/image.png')",
            vec![
                Token::Function("url".to_owned()),
                Token::String("https://example.com/image.png".to_owned()),
                Token::CloseParenthesis(),
            ],
        );
    }

    #[test]
    fn url_with_whitespace() {
        assert_tokens(
            "url(   \"https://example.com/image.png\"   )",
            vec![
                Token::Function("url".to_owned()),
                // according to the spec there should be whitespace here but we omit it to make parsing easier
                Token::String("https://example.com/image.png".to_owned()),
                Token::Whitespace(),
                Token::CloseParenthesis(),
            ],
        );
    }
}

mod urls {
    use super::*;

    #[test]
    fn empty() {
        assert_tokens("url()", vec![Token::BadUrl(), Token::CloseParenthesis()]);
    }

    #[test]
    fn one_character() {
        assert_tokens("url(a)", vec![Token::Url("a".to_owned())]);
    }

    #[test]
    fn example_image() {
        assert_tokens(
            "url(https://example.com/image.png)",
            vec![Token::Url("https://example.com/image.png".to_owned())],
        );
    }

    #[test]
    fn with_whitespace() {
        assert_tokens(
            "url(   https://example.com/image.png   )",
            vec![Token::Url("https://example.com/image.png".to_owned())],
        );
    }

    #[test]
    fn upper_case() {
        assert_tokens(
            "URL(https://example.com/image.png)",
            vec![Token::Url("https://example.com/image.png".to_owned())],
        );
    }

    #[test]
    fn mixed_case() {
        assert_tokens(
            "uRL(https://example.com/image.png)",
            vec![Token::Url("https://example.com/image.png".to_owned())],
        );
    }

    #[test]
    fn interrupted_by_whitespace() {
        assert_tokens(
            "url(https://url.with spaces)",
            vec![
                Token::BadUrl(),
                // according to the spec there should be whitespace here but we omit it to make parsing easier
                Token::Identifier("spaces".to_owned()),
                Token::CloseParenthesis(),
            ],
        );
    }

    #[test]
    fn interrupted_by_double_quote() {
        assert_tokens(
            "url(https://url.with.quote\")",
            vec![Token::BadUrl(), Token::BadString()],
        );
    }

    #[test]
    fn interrupted_by_single_quote() {
        assert_tokens(
            "url(https://url.with.quote')",
            vec![Token::BadUrl(), Token::BadString()],
        );
    }

    #[test]
    fn interrupted_by_open_paren() {
        assert_tokens(
            "url(https://url.with.paren()",
            vec![
                Token::BadUrl(),
                Token::OpenParenthesis(),
                Token::CloseParenthesis(),
            ],
        );
    }

    #[test]
    fn interrupted_by_null() {
        assert_tokens(
            "url(https://url.with.null\0)",
            vec![
                Token::BadUrl(),
                Token::Delimiter('\0'),
                Token::CloseParenthesis(),
            ],
        );
    }
}

mod at_keywords {
    use super::*;

    #[test]
    fn at_media() {
        assert_tokens("@media", vec![Token::AtKeyword("media".to_owned())]);
    }

    #[test]
    fn at_import() {
        assert_tokens("@import", vec![Token::AtKeyword("import".to_owned())]);
    }

    #[test]
    fn delimiter() {
        assert_tokens("@", vec![Token::Delimiter('@')]);
    }
}

mod hashes {
    use super::*;

    #[test]
    fn delimiter() {
        assert_tokens("#", vec![Token::Delimiter('#')]);
    }

    #[test]
    fn one_letter() {
        assert_tokens("#a", vec![Token::Hash("a".to_owned(), HashType::Id)]);
    }

    #[test]
    fn three_letters() {
        assert_tokens("#abc", vec![Token::Hash("abc".to_owned(), HashType::Id)]);
    }

    #[test]
    fn four_letters() {
        assert_tokens("#abcd", vec![Token::Hash("abcd".to_owned(), HashType::Id)]);
    }

    #[test]
    fn one_digit() {
        assert_tokens(
            "#1",
            vec![Token::Hash("1".to_owned(), HashType::Unrestricted)],
        );
    }

    #[test]
    fn four_digits() {
        assert_tokens(
            "#1234",
            vec![Token::Hash("1234".to_owned(), HashType::Unrestricted)],
        );
    }

    #[test]
    fn followed_by_whitespace() {
        assert_tokens(
            "#abc ",
            vec![
                Token::Hash("abc".to_owned(), HashType::Id),
                Token::Whitespace(),
            ],
        );
    }

    #[test]
    fn followed_by_colon() {
        assert_tokens(
            "#abc:",
            vec![Token::Hash("abc".to_owned(), HashType::Id), Token::Colon()],
        );
    }
}

mod strings {
    use super::*;

    #[test]
    fn empty_double() {
        assert_tokens("\"\"", vec![Token::String("".to_owned())]);
    }

    #[test]
    fn empty_single() {
        assert_tokens("''", vec![Token::String("".to_owned())]);
    }

    #[test]
    fn one_character_double() {
        assert_tokens("\"a\"", vec![Token::String("a".to_owned())]);
    }

    #[test]
    fn one_character_single() {
        assert_tokens("'a'", vec![Token::String("a".to_owned())]);
    }

    #[test]
    fn many_characters_double() {
        assert_tokens("\"abc def\"", vec![Token::String("abc def".to_owned())]);
    }

    #[test]
    fn many_characters_single() {
        assert_tokens("'abc def'", vec![Token::String("abc def".to_owned())]);
    }

    #[test]
    fn special_characters() {
        assert_tokens(
            "\"!@#$%^&*-+=;:,.?/`~|()[]{}\"",
            vec![Token::String("!@#$%^&*-+=;:,.?/`~|()[]{}".to_owned())],
        );
    }

    #[test]
    fn escaped_double() {
        assert_tokens("\"\\\"\"", vec![Token::String("\"".to_owned())]);
    }

    #[test]
    fn escaped_single() {
        assert_tokens("'\\''", vec![Token::String("'".to_owned())]);
    }

    #[test]
    fn escaped_newline() {
        assert_tokens("\"\\\n\"", vec![Token::String("".to_owned())]);
    }

    #[test]
    fn no_close() {
        assert_tokens("\"abc", vec![Token::BadString()]);
    }

    #[test]
    fn interrupted_by_newline() {
        assert_tokens("\"abc\n", vec![Token::BadString()]);
    }
}

mod numbers {
    use super::*;

    #[test]
    fn zero() {
        assert_tokens("0", vec![Token::Number(0.0)]);
    }

    #[test]
    fn one() {
        assert_tokens("1", vec![Token::Number(1.0)]);
    }

    #[test]
    fn negative_one() {
        assert_tokens("-1", vec![Token::Number(-1.0)]);
    }

    #[test]
    fn positive_one() {
        assert_tokens("+1", vec![Token::Number(1.0)]);
    }

    #[test]
    fn one_point_zero() {
        assert_tokens("1.0", vec![Token::Number(1.0)]);
    }

    #[test]
    fn one_point_five() {
        assert_tokens("1.5", vec![Token::Number(1.5)]);
    }

    #[test]
    fn point_five() {
        assert_tokens(".5", vec![Token::Number(0.5)]);
    }

    #[test]
    fn zero_percent() {
        assert_tokens("0%", vec![Token::Percentage(0.0)]);
    }

    #[test]
    fn one_percent() {
        assert_tokens("1%", vec![Token::Percentage(1.0)]);
    }

    #[test]
    fn one_hundred_percent() {
        assert_tokens("100%", vec![Token::Percentage(100.0)]);
    }

    #[test]
    fn negative_three_hundred_percent() {
        assert_tokens("-300%", vec![Token::Percentage(-300.0)]);
    }

    #[test]
    fn positive_point_five_percent() {
        assert_tokens("+.5%", vec![Token::Percentage(0.5)]);
    }

    #[test]
    fn zero_px() {
        assert_tokens("0px", vec![Token::Dimension(0.0, "px".to_owned())]);
    }

    #[test]
    fn negative_three_em() {
        assert_tokens("-3em", vec![Token::Dimension(-3.0, "em".to_owned())]);
    }
}

mod whitespace {
    use super::*;

    #[test]
    fn one_space() {
        assert_tokens(" ", vec![Token::Whitespace()]);
    }

    #[test]
    fn many_spaces() {
        assert_tokens("   ", vec![Token::Whitespace()]);
    }

    #[test]
    fn one_tab() {
        assert_tokens("\t", vec![Token::Whitespace()]);
    }

    #[test]
    fn newline() {
        assert_tokens("\n", vec![Token::Whitespace()]);
    }

    #[test]
    fn carriage_return() {
        assert_tokens("\r", vec![Token::Whitespace()]);
    }

    #[test]
    fn many_characters() {
        assert_tokens(
            "\t\t\t        \r\n   \r\n  \n \r\r\t",
            vec![Token::Whitespace()],
        );
    }
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
fn complex_file() {
    assert_tokens(
        "
        /* example css file */
        @import url(\"https://fonts.googleapis.com/css2?family=Roboto&display=swap\");
        @import url(https://fonts.googleapis.com/css2?family=Roboto+Mono&display=swap);

        :root {
            --color-primary: #880000;
            font-family: 'Roboto', sans-serif;
        }

        #header {
            background-color: var(--color-primary);
            color: rgb(255, 255, 255);
        }

        @media (min-width: 600px) {
            *[role=\"main\"] {
                max-width: 75%;
            }
        }

        /* invalid comment because it is not closed
        ",
        vec![
            Token::Whitespace(),
            Token::Whitespace(),
            Token::AtKeyword("import".to_owned()),
            Token::Whitespace(),
            Token::Function("url".to_owned()),
            Token::String(
                "https://fonts.googleapis.com/css2?family=Roboto&display=swap".to_owned(),
            ),
            Token::CloseParenthesis(),
            Token::Semicolon(),
            Token::Whitespace(),
            Token::AtKeyword("import".to_owned()),
            Token::Whitespace(),
            Token::Url(
                "https://fonts.googleapis.com/css2?family=Roboto+Mono&display=swap".to_owned(),
            ),
            Token::Semicolon(),
            Token::Whitespace(),
            Token::Colon(),
            Token::Identifier("root".to_owned()),
            Token::Whitespace(),
            Token::OpenCurlyBracket(),
            Token::Whitespace(),
            Token::Identifier("--color-primary".to_owned()),
            Token::Colon(),
            Token::Whitespace(),
            Token::Hash("880000".to_owned(), HashType::Unrestricted),
            Token::Semicolon(),
            Token::Whitespace(),
            Token::Identifier("font-family".to_owned()),
            Token::Colon(),
            Token::Whitespace(),
            Token::String("Roboto".to_owned()),
            Token::Comma(),
            Token::Whitespace(),
            Token::Identifier("sans-serif".to_owned()),
            Token::Semicolon(),
            Token::Whitespace(),
            Token::CloseCurlyBracket(),
            Token::Whitespace(),
            Token::Hash("header".to_owned(), HashType::Id),
            Token::Whitespace(),
            Token::OpenCurlyBracket(),
            Token::Whitespace(),
            Token::Identifier("background-color".to_owned()),
            Token::Colon(),
            Token::Whitespace(),
            Token::Function("var".to_owned()),
            Token::Identifier("--color-primary".to_owned()),
            Token::CloseParenthesis(),
            Token::Semicolon(),
            Token::Whitespace(),
            Token::Identifier("color".to_owned()),
            Token::Colon(),
            Token::Whitespace(),
            Token::Function("rgb".to_owned()),
            Token::Number(255.0),
            Token::Comma(),
            Token::Whitespace(),
            Token::Number(255.0),
            Token::Comma(),
            Token::Whitespace(),
            Token::Number(255.0),
            Token::CloseParenthesis(),
            Token::Semicolon(),
            Token::Whitespace(),
            Token::CloseCurlyBracket(),
            Token::Whitespace(),
            Token::AtKeyword("media".to_owned()),
            Token::Whitespace(),
            Token::OpenParenthesis(),
            Token::Identifier("min-width".to_owned()),
            Token::Colon(),
            Token::Whitespace(),
            Token::Dimension(600.0, "px".to_owned()),
            Token::CloseParenthesis(),
            Token::Whitespace(),
            Token::OpenCurlyBracket(),
            Token::Whitespace(),
            Token::Delimiter('*'),
            Token::OpenSquareBracket(),
            Token::Identifier("role".to_owned()),
            Token::Delimiter('='),
            Token::String("main".to_owned()),
            Token::CloseSquareBracket(),
            Token::Whitespace(),
            Token::OpenCurlyBracket(),
            Token::Whitespace(),
            Token::Identifier("max-width".to_owned()),
            Token::Colon(),
            Token::Whitespace(),
            Token::Percentage(75.0),
            Token::Semicolon(),
            Token::Whitespace(),
            Token::CloseCurlyBracket(),
            Token::Whitespace(),
            Token::CloseCurlyBracket(),
            Token::Whitespace(),
            Token::BadComment(),
        ],
    );
}
