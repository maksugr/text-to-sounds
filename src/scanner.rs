/// The scanner to move through the text
/// Inspired by [Lyn crate](https://crates.io/crates/lyn) and by an [article](https://depth-first.com/articles/2021/12/16/a-beginners-guide-to-parsing-in-rust/)
#[derive(Debug)]
pub struct Scanner {
    cursor: usize,
    characters: Vec<char>,
}

/// Default letter if Scanner will found nothing
/// Just an easy workaround for Option
const DEFAULT_LETTER: &char = &' ';

impl Scanner {
    /// Creates new Scanner
    pub fn new(string: &str) -> Self {
        Self {
            cursor: 0,
            characters: string.chars().collect(),
        }
    }

    /// Returns the current cursor. Useful for reporting errors.
    #[allow(dead_code)]
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Returns the next character without advancing the cursor.
    pub fn peek(&self) -> &char {
        self.characters.get(self.cursor).unwrap_or(DEFAULT_LETTER)
    }

    /// Returns the next + 1 character without advancing the cursor.
    pub fn peek_next(&self) -> &char {
        self.characters
            .get(self.cursor + 1)
            .unwrap_or(DEFAULT_LETTER)
    }

    /// Returns true if further progress is not possible.
    pub fn is_done(&self) -> bool {
        self.cursor == self.characters.len()
    }

    /// Returns true if the first char.
    pub fn is_first(&self) -> bool {
        self.cursor == 0
    }

    /// Returns true if the last char.
    pub fn is_last(&self) -> bool {
        self.cursor + 1 == self.characters.len()
    }

    /// Returns the next character and advances the cursor.
    pub fn pop(&mut self) -> &char {
        match self.characters.get(self.cursor) {
            Some(character) => {
                self.cursor += 1;

                character
            }
            None => DEFAULT_LETTER,
        }
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

        assert_eq!(scanner.peek(), DEFAULT_LETTER)
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

        assert_eq!(scanner.peek_next(), DEFAULT_LETTER)
    }

    #[test]
    fn not_done() {
        let mut scanner = Scanner::new("abc");

        scanner.pop();

        assert_eq!(scanner.peek_next(), &'c')
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
    fn not_is_last() {
        let scanner = Scanner::new("abc");

        assert!(!scanner.is_last())
    }

    #[test]
    fn is_last() {
        let mut scanner = Scanner::new("abc");

        scanner.pop();
        scanner.pop();

        assert!(scanner.is_last())
    }
}

#[cfg(test)]
mod pop {
    use super::*;

    #[test]
    fn empty() {
        let mut scanner = Scanner::new("");

        assert_eq!(scanner.pop(), DEFAULT_LETTER);
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

        assert_eq!(scanner.pop(), DEFAULT_LETTER);
        assert_eq!(scanner.cursor(), 3)
    }
}
