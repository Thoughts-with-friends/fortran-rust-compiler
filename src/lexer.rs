use anyhow::{anyhow, bail, Context, Result};

#[cfg(test)]
use pretty_assertions::assert_eq;

use crate::error::ErrorKind;

macro_rules! lexer_test {
    (FAIL: $name:ident, $func:ident, $src:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let src: &str = $src;
            let func = $func;

            let got = func(src);
            assert!(got.is_err(), "{:?} should be an error", got);
        }
    };
    ($name:ident, $func:ident, $src:expr => $should_be:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let src: &str = $src;
            let should_be = TokenKind::from($should_be);
            let func = $func;

            let (got, _bytes_read) = func(src).unwrap();
            assert_eq!(got, should_be, "Input was {:?}", src);
        }
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Integer(usize),
    Decimal(f64),
    Plus,
}

impl From<usize> for TokenKind {
    fn from(other: usize) -> TokenKind {
        TokenKind::Integer(other)
    }
}

impl From<f64> for TokenKind {
    fn from(other: f64) -> TokenKind {
        TokenKind::Decimal(other)
    }
}

fn take_while<F>(data: &str, mut pred: F) -> Result<(&str, usize)>
where
    F: FnMut(char) -> bool,
{
    let mut current_index = 0;

    for ch in data.chars() {
        let should_continue = pred(ch);

        if !should_continue {
            break;
        }

        current_index += ch.len_utf8();
    }

    if current_index == 0 {
        bail!("No characters matched")
    } else {
        Ok((&data[..current_index], current_index))
    }
}

// Tokenize a numeric literal.
fn tokenize_number(data: &str) -> Result<(TokenKind, usize)> {
    let mut seen_dot = false;

    let (decimal, bytes_read) = take_while(data, |c| {
        if c.is_digit(10) {
            true
        } else if c == '.' {
            if !seen_dot {
                seen_dot = true;
                true
            } else {
                false
            }
        } else {
            false
        }
    })?;

    if seen_dot {
        let n: f64 = decimal.parse()?;
        Ok((TokenKind::Decimal(n), bytes_read))
    } else {
        let n: usize = decimal.parse()?;
        Ok((TokenKind::Integer(n), bytes_read))
    }
}

lexer_test!(tokenize_a_single_digit_integer, tokenize_number, "1" => 1);
lexer_test!(tokenize_a_longer_integer, tokenize_number, "1234567890" => 1234567890);
lexer_test!(tokenize_basic_decimal, tokenize_number, "12.3" => 12.3);
lexer_test!(tokenize_string_with_multiple_decimal_points, tokenize_number, "12.3.456" => 12.3);
lexer_test!(
    FAIL: cant_tokenize_a_string_as_a_decimal,
    tokenize_number,
    "asdfghj"
);
lexer_test!(tokenizing_decimal_stops_at_alpha, tokenize_number, "123.4asdfghj" => 123.4);

fn skip_whitespace(data: &str) -> usize {
    match take_while(data, |ch| ch.is_whitespace()) {
        Ok((_, bytes_skipped)) => bytes_skipped,
        _ => 0,
    }
}

#[test]
fn skip_past_several_whitespace_chars() {
    let src = " \t\n\r123";
    let should_be = 4;

    let num_skipped = skip_whitespace(src);
    assert_eq!(num_skipped, should_be);
}

#[test]
fn skipping_whitespace_when_first_is_a_letter_returns_zero() {
    let src = "Hello World";
    let should_be = 0;

    let num_skipped = skip_whitespace(src);
    assert_eq!(num_skipped, should_be);
}

fn skip_comments(src: &str) -> usize {
    let pairs = [("!", "\n")];

    for &(pattern, matcher) in &pairs {
        if src.starts_with(pattern) {
            let leftovers = skip_until(src, matcher);
            return src.len() - leftovers.len();
        }
    }

    0
}

fn skip_until<'a>(mut src: &'a str, pattern: &str) -> &'a str {
    while !src.is_empty() && !src.starts_with(pattern) {
        let next_char_size = src
            .chars()
            .next()
            .expect("The string isn't empty")
            .len_utf8();
        src = &src[next_char_size..];
    }

    &src[pattern.len()..]
}

macro_rules! comment_test {
    ($name:ident, $src:expr => $should_be:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let got = skip_comments($src);
            assert_eq!(got, $should_be);
        }
    };
}

comment_test!(slash_slash_skips_to_end_of_line, "! foo bar { baz }\n 1234" => 18);

/// Skip past any whitespace characters or comments.
fn skip(src: &str) -> usize {
    let mut remaining = src;

    loop {
        let ws = skip_whitespace(remaining);
        remaining = &remaining[ws..];
        let comments = skip_comments(remaining);
        remaining = &remaining[comments..];

        if ws + comments == 0 {
            return src.len() - remaining.len();
        }
    }
}

/// Try to lex a single token from the input stream.
pub fn tokenize_single_token(data: &str) -> Result<(TokenKind, usize)> {
    let next = match data.chars().next() {
        Some(c) => c,
        None => bail!(anyhow!(ErrorKind::UnexpectedEOF)),
    };

    let (tok, length) = match next {
        '+' => (TokenKind::Plus, 1),
        '0'..='9' => tokenize_number(data).with_context(|| "Couldn't tokenize a number")?,
        other => bail!(ErrorKind::UnknownCharacter(other)),
    };

    Ok((tok, length))
}

struct Tokenizer<'a> {
    current_index: usize,
    remaining_text: &'a str,
}

impl<'a> Tokenizer<'a> {
    fn new(src: &str) -> Tokenizer {
        Tokenizer {
            current_index: 0,
            remaining_text: src,
        }
    }

    fn next_token(&mut self) -> Result<Option<(TokenKind, usize, usize)>> {
        self.skip_whitespace();

        if self.remaining_text.is_empty() {
            Ok(None)
        } else {
            let start = self.current_index;
            let tok = self._next_token().with_context(|| {
                ErrorKind::MessageWithLocation(self.current_index, "Couldn't read the next token")
            })?;
            let end = self.current_index;
            Ok(Some((tok, start, end)))
        }
    }

    fn skip_whitespace(&mut self) {
        let skipped = skip(self.remaining_text);
        self.chomp(skipped);
    }

    fn _next_token(&mut self) -> Result<TokenKind> {
        let (tok, bytes_read) = tokenize_single_token(self.remaining_text)?;
        self.chomp(bytes_read);

        Ok(tok)
    }

    fn chomp(&mut self, num_bytes: usize) {
        self.remaining_text = &self.remaining_text[num_bytes..];
        self.current_index += num_bytes;
    }
}

/// Turn a string of valid Delphi code into a list of tokens, including the
/// location of that token's start and end point in the original source code.
///
/// Note the token indices represent the half-open interval `[start, end)`,
/// equivalent to `start .. end` in Rust.
pub fn tokenize(src: &str) -> Result<Vec<(TokenKind, usize, usize)>> {
    let mut tokenizer = Tokenizer::new(src);
    let mut tokens = Vec::new();

    while let Some(tok) = tokenizer.next_token()? {
        tokens.push(tok);
    }

    Ok(tokens)
}
