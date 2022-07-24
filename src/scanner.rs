/// The scanner to move through the text
/// Inspired by [Lyn crate](https://crates.io/crates/lyn) and by an [article](https://depth-first.com/articles/2021/12/16/a-beginners-guide-to-parsing-in-rust/)
#[derive(Debug)]
pub struct Scanner {
    cursor: usize,
    characters: Vec<char>,
}

/// Space html character
const SPACE_HTML_CHAR: &char = &' ';

/// Non-breakable space html character
const NON_BREAKABLE_SPACE_HTML_CHAR: &char = &'\u{a0}';

/// Default char if Scanner will found nothing
/// Just an easy workaround for Option
const DEFAULT_CHAR: &char = SPACE_HTML_CHAR;

/// Array of the punctuation characters
const PUNCTUATION_CHARS: [char; 7] = ['.', ',', ';', '!', '?', ':', '-'];

impl Scanner {
    /// Creates new Scanner
    pub fn new(string: &str) -> Self {
        Self {
            cursor: 0,
            characters: string.chars().collect(),
        }
    }

    /// Returns the current cursor. Useful for reporting errors.
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Returns the next character without advancing the cursor.
    pub fn peek(&self) -> &char {
        self.characters.get(self.cursor).unwrap_or(DEFAULT_CHAR)
    }

    /// Returns the next + 1 character without advancing the cursor.
    pub fn peek_next(&self) -> &char {
        self.characters.get(self.cursor + 1).unwrap_or(DEFAULT_CHAR)
    }

    /// Returns the prev character without advancing the cursor.
    pub fn peek_prev(&self) -> &char {
        match self.cursor() == 0 {
            true => DEFAULT_CHAR,
            false => self.characters.get(self.cursor - 1).unwrap_or(DEFAULT_CHAR),
        }
    }

    /// Returns true if further progress is not possible.
    pub fn is_done(&self) -> bool {
        self.cursor == self.characters.len()
    }

    /// Returns true if the first char.
    pub fn is_first(&self) -> bool {
        match self.cursor == 0 {
            true => true,
            false => {
                let prev_char = self.peek_prev();

                prev_char == SPACE_HTML_CHAR
                    || prev_char == NON_BREAKABLE_SPACE_HTML_CHAR
                    || Self::is_punctuation(prev_char)
            }
        }
    }

    /// Returns true if the last char.
    pub fn is_last(&self) -> bool {
        match self.cursor + 1 == self.characters.len() {
            true => true,
            false => {
                let next_char = self.peek_next();

                next_char == SPACE_HTML_CHAR
                    || next_char == NON_BREAKABLE_SPACE_HTML_CHAR
                    || Self::is_punctuation(next_char)
            }
        }
    }

    // Returns true if next char exists in `chars` param
    pub fn is_next_any(&self, chars: Vec<char>) -> bool {
        chars.iter().any(|c| self.peek_next() == c)
    }

    /// Returns the next character and advances the cursor.
    pub fn pop(&mut self) -> &char {
        match self.characters.get(self.cursor) {
            Some(character) => {
                self.cursor += 1;

                character
            }
            None => DEFAULT_CHAR,
        }
    }

    /// Returns true if the character is a punctuation character
    fn is_punctuation(c: &char) -> bool {
        PUNCTUATION_CHARS.iter().any(|cc| cc == c)
    }
}

#[cfg(test)]
mod cursor {
    use super::*;

    #[test]
    fn empty() {
        let scanner = Scanner::new("");

        assert_eq!(scanner.cursor(), 0)
    }

    #[test]
    fn in_progress() {
        let mut scanner = Scanner::new("abc");

        scanner.pop();

        assert_eq!(scanner.cursor(), 1);
    }

    #[test]
    fn done() {
        let mut scanner = Scanner::new("abc");

        scanner.pop();
        scanner.pop();
        scanner.pop();

        assert_eq!(scanner.cursor(), 3)
    }
}

#[cfg(test)]
mod is_done {
    use super::*;

    #[test]
    fn emtpy() {
        let scanner = Scanner::new("");

        assert!(scanner.is_done())
    }

    #[test]
    fn not_done() {
        let mut scanner = Scanner::new("abc");

        scanner.pop();

        assert!(!scanner.is_done())
    }

    #[test]
    fn done() {
        let mut scanner = Scanner::new("abc");

        scanner.pop();
        scanner.pop();
        scanner.pop();

        assert!(scanner.is_done())
    }
}

#[cfg(test)]
mod peek {
    use super::*;

    #[test]
    fn empty() {
        let scanner = Scanner::new("");

        assert_eq!(scanner.peek(), DEFAULT_CHAR)
    }

    #[test]
    fn not_done() {
        let mut scanner = Scanner::new("abc");

        scanner.pop();

        assert_eq!(scanner.peek(), &'b')
    }
}

#[cfg(test)]
mod peek_next {
    use super::*;

    #[test]
    fn empty() {
        let scanner = Scanner::new("");

        assert_eq!(scanner.peek_next(), DEFAULT_CHAR)
    }

    #[test]
    fn not_done() {
        let mut scanner = Scanner::new("abc");

        scanner.pop();

        assert_eq!(scanner.peek_next(), &'c')
    }
}

#[cfg(test)]
mod peek_prev {
    use super::*;

    #[test]
    fn empty() {
        let scanner = Scanner::new("");

        assert_eq!(scanner.peek_prev(), DEFAULT_CHAR)
    }

    #[test]
    fn not_done() {
        let mut scanner = Scanner::new("abc");

        scanner.pop();

        assert_eq!(scanner.peek_prev(), &'a')
    }
}

#[cfg(test)]
mod is_first {
    use super::*;

    #[test]
    fn is_first() {
        let scanner = Scanner::new("abc");

        assert!(scanner.is_first())
    }

    #[test]
    fn is_first_with_punctuation_char() {
        let mut scanner = Scanner::new("!abc");

        scanner.pop();

        assert!(scanner.is_first())
    }

    #[test]
    fn is_first_with_non_breakable_char() {
        let mut scanner = Scanner::new("\u{a0}abc");

        scanner.pop();

        assert!(scanner.is_first())
    }

    #[test]
    fn not_is_first() {
        let mut scanner = Scanner::new("abc");

        scanner.pop();

        assert!(!scanner.is_first())
    }
}

#[cfg(test)]
mod is_last {
    use super::*;

    #[test]
    fn is_last() {
        let mut scanner = Scanner::new("abc");

        scanner.pop();
        scanner.pop();

        assert!(scanner.is_last())
    }

    #[test]
    fn is_last_with_punctuation_char() {
        let mut scanner = Scanner::new("abc!");

        scanner.pop();
        scanner.pop();

        assert!(scanner.is_last())
    }

    #[test]
    fn is_last_with_non_breakable_char() {
        let mut scanner = Scanner::new("abc\u{a0}");

        scanner.pop();
        scanner.pop();

        assert!(scanner.is_last())
    }

    #[test]
    fn not_is_last() {
        let scanner = Scanner::new("abc");

        assert!(!scanner.is_last())
    }
}

#[cfg(test)]
mod is_next_any {
    use super::*;

    #[test]
    fn it_should_be_true() {
        let scanner = Scanner::new("cheese");

        assert!(scanner.is_next_any(vec!['h']));
    }

    #[test]
    fn it_should_be_false() {
        let scanner = Scanner::new("cheese");

        assert!(!scanner.is_next_any(vec!['c']));
    }
}

#[cfg(test)]
mod pop {
    use super::*;

    #[test]
    fn empty() {
        let mut scanner = Scanner::new("");

        assert_eq!(scanner.pop(), DEFAULT_CHAR);
        assert_eq!(scanner.cursor(), 0)
    }

    #[test]
    fn not_done() {
        let mut scanner = Scanner::new("abc");

        assert_eq!(scanner.pop(), &'a');
        assert_eq!(scanner.cursor(), 1)
    }

    #[test]
    fn done() {
        let mut scanner = Scanner::new("abc");

        scanner.pop();
        scanner.pop();
        scanner.pop();

        assert_eq!(scanner.pop(), DEFAULT_CHAR);
        assert_eq!(scanner.cursor(), 3)
    }
}

#[cfg(test)]
mod is_punctuation {
    use super::*;

    #[test]
    fn is_punctuation() {
        assert!(Scanner::is_punctuation(&'!'));
    }

    #[test]
    fn is_not_punctuation() {
        assert!(!Scanner::is_punctuation(&'k'));
    }
}
