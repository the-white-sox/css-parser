pub struct LineCounter<I: Iterator<Item = char>> {
    line: usize,
    column: usize,
    chars: I,
}

impl<I: Iterator<Item = char>> LineCounter<I> {
    pub fn new(chars: I) -> Self {
        Self {
            line: 0,
            column: 0,
            chars,
        }
    }
}

impl<I: Iterator<Item = char>> Iterator for LineCounter<I> {
    type Item = (usize, usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.line;
        let column = self.column;
        match self.chars.next() {
            Some('\n') => {
                self.line += 1;
                self.column = 0;
                Some((line, column, '\n'))
            }
            Some(character) => {
                self.column += 1;
                Some((line, column, character))
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        let input = "";
        let mut line_counter = LineCounter::new(input.chars());
        assert_eq!(line_counter.next(), None);
    }

    #[test]
    fn one_character() {
        let input = "a";
        let mut line_counter = LineCounter::new(input.chars());
        assert_eq!(line_counter.next(), Some((0, 0, 'a')));
        assert_eq!(line_counter.next(), None);
    }

    #[test]
    fn one_row() {
        let input = "abc";
        let mut line_counter = LineCounter::new(input.chars());
        assert_eq!(line_counter.next(), Some((0, 0, 'a')));
        assert_eq!(line_counter.next(), Some((0, 1, 'b')));
        assert_eq!(line_counter.next(), Some((0, 2, 'c')));
        assert_eq!(line_counter.next(), None);
    }

    #[test]
    fn two_rows() {
        let input = "abc\ndef";
        let mut line_counter = LineCounter::new(input.chars());
        assert_eq!(line_counter.next(), Some((0, 0, 'a')));
        assert_eq!(line_counter.next(), Some((0, 1, 'b')));
        assert_eq!(line_counter.next(), Some((0, 2, 'c')));
        assert_eq!(line_counter.next(), Some((0, 3, '\n')));
        assert_eq!(line_counter.next(), Some((1, 0, 'd')));
        assert_eq!(line_counter.next(), Some((1, 1, 'e')));
        assert_eq!(line_counter.next(), Some((1, 2, 'f')));
        assert_eq!(line_counter.next(), None);
    }

    #[test]
    fn two_rows_with_carriage_return() {
        let input = "abc\r\ndef";
        let mut line_counter = LineCounter::new(input.chars());
        assert_eq!(line_counter.next(), Some((0, 0, 'a')));
        assert_eq!(line_counter.next(), Some((0, 1, 'b')));
        assert_eq!(line_counter.next(), Some((0, 2, 'c')));
        assert_eq!(line_counter.next(), Some((0, 3, '\r')));
        assert_eq!(line_counter.next(), Some((0, 4, '\n')));
        assert_eq!(line_counter.next(), Some((1, 0, 'd')));
        assert_eq!(line_counter.next(), Some((1, 1, 'e')));
        assert_eq!(line_counter.next(), Some((1, 2, 'f')));
        assert_eq!(line_counter.next(), None);
    }

    #[test]
    fn trailing_newline() {
        let input = "abc\ndef\n";
        let mut line_counter = LineCounter::new(input.chars());
        assert_eq!(line_counter.next(), Some((0, 0, 'a')));
        assert_eq!(line_counter.next(), Some((0, 1, 'b')));
        assert_eq!(line_counter.next(), Some((0, 2, 'c')));
        assert_eq!(line_counter.next(), Some((0, 3, '\n')));
        assert_eq!(line_counter.next(), Some((1, 0, 'd')));
        assert_eq!(line_counter.next(), Some((1, 1, 'e')));
        assert_eq!(line_counter.next(), Some((1, 2, 'f')));
        assert_eq!(line_counter.next(), Some((1, 3, '\n')));
        assert_eq!(line_counter.next(), None);
    }
}
