use crate::scanner::Scanner;

pub fn any_letter(letters: Vec<char>, scanner: &Scanner) -> bool {
    letters.iter().any(|letter| scanner.peek_next() == letter)
}

#[cfg(test)]
mod any_letter {
    use super::{any_letter, Scanner};

    #[test]
    fn it_should_be_true() {
        let scanner = Scanner::new("cheese");

        assert!(any_letter(vec!['h'], &scanner));
    }

    #[test]
    fn it_should_be_false() {
        let scanner = Scanner::new("cheese");

        assert!(!any_letter(vec!['c'], &scanner));
    }
}
