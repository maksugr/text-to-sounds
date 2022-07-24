use crate::scanner::Scanner;
use crate::sound::SoundKind;
use crate::utils::any_letter;

const NON_BREAKABLE_HTML_CHAR: char = '\u{a0}';
const PUNCTUATION_CHARS: [char; 7] = ['.', ',', ';', '!', '?', ':', '-'];

fn is_punctuation(c: &char) -> bool {
    PUNCTUATION_CHARS.iter().any(|cc| cc == c)
}

fn is_first(scanner: &Scanner) -> bool {
    let prev_char = scanner.peek_prev();

    scanner.is_first()
        || prev_char == &' '
        || prev_char == &NON_BREAKABLE_HTML_CHAR
        || is_punctuation(prev_char)
}

fn is_last(scanner: &Scanner) -> bool {
    let next_char = scanner.peek_next();

    scanner.is_last()
        || next_char == &' '
        || next_char == &NON_BREAKABLE_HTML_CHAR
        || is_punctuation(next_char)
}

fn highlight_two_letters(
    first_letter: &char,
    second_letter: &char,
    kind: SoundKind,
    result_text: &mut String,
) {
    let text = first_letter.to_string() + &String::from(*second_letter);

    result_text.push_str(&format!("<span class='{:?}'>{}</span>", kind, text));
}

/// Highlight sounds in the text with html tags
///
/// ## Example
///
/// ```rust
/// use text_to_sounds::{highlight};
///
/// assert_eq!(highlight("The text just in case"), "<span class='Th'>Th</span>e <span class='Ptk'>t</span>ex<span class='Ptk'>t</span> <span class='Dj'>j</span>us<span class='Ptk'>t</span> in <span class='Ptk'>c</span>ase".to_string());
/// ```
pub fn highlight<T: AsRef<str>>(text: T) -> String {
    let text = text.as_ref();

    if text.is_empty() {
        return text.to_string();
    }

    let mut scanner = Scanner::new(text);

    let mut result_text = String::new();

    while !scanner.is_done() {
        match scanner.peek() {
            letter @ ('c' | 'C') if !is_last(&scanner) && any_letter(vec!['h', 'H'], &scanner) => {
                let next_letter = scanner.peek_next();

                highlight_two_letters(letter, next_letter, SoundKind::Ch, &mut result_text);

                scanner.pop();
                scanner.pop();
            }
            letter @ ('p' | 'P' | 't' | 'T' | 'c' | 'C')
                if is_first(&scanner) || is_last(&scanner) =>
            {
                if (letter == &'t' || letter == &'T')
                    && !is_last(&scanner)
                    && any_letter(vec!['h', 'H'], &scanner)
                {
                    let next_letter = scanner.peek_next();

                    highlight_two_letters(letter, next_letter, SoundKind::Th, &mut result_text);

                    scanner.pop();
                    scanner.pop();

                    continue;
                }

                result_text.push_str(&format!(
                    "<span class='{:?}'>{}</span>",
                    SoundKind::Ptk,
                    letter
                ));

                scanner.pop();
            }
            letter @ ('t' | 'T') if any_letter(vec!['h', 'H'], &scanner) => {
                let next_letter = scanner.peek_next();

                highlight_two_letters(letter, next_letter, SoundKind::Th, &mut result_text);

                scanner.pop();
                scanner.pop();
            }
            letter @ ('w' | 'W') if is_first(&scanner) => {
                result_text.push_str(&format!(
                    "<span class='{:?}'>{}</span>",
                    SoundKind::W,
                    letter
                ));

                scanner.pop();
            }
            letter @ ('v' | 'V') if is_first(&scanner) => {
                result_text.push_str(&format!(
                    "<span class='{:?}'>{}</span>",
                    SoundKind::V,
                    letter
                ));

                scanner.pop();
            }
            letter @ ('n' | 'N')
                if !is_last(&scanner) && any_letter(vec!['g', 'G', 'k', 'K'], &scanner) =>
            {
                let next_letter = scanner.peek_next();

                highlight_two_letters(letter, next_letter, SoundKind::Ng, &mut result_text);

                scanner.pop();
                scanner.pop();
            }
            letter @ ('j' | 'J') if is_first(&scanner) => {
                result_text.push_str(&format!(
                    "<span class='{:?}'>{}</span>",
                    SoundKind::Dj,
                    letter
                ));

                scanner.pop();
            }
            _ => {
                let letter = scanner.pop();
                result_text.push_str(&letter.to_string());
            }
        }
    }

    result_text
}

#[cfg(test)]
mod highlight {
    use super::highlight;

    #[test]
    fn it_should_highlight_ptk() {
        assert_eq!(highlight("Put a cat"), "<span class='Ptk'>P</span>u<span class='Ptk'>t</span> a <span class='Ptk'>c</span>a<span class='Ptk'>t</span>".to_string());
    }

    #[test]
    fn it_should_highlight_ptk_with_space() {
        assert_eq!(
            highlight("pp "),
            "<span class='Ptk'>p</span><span class='Ptk'>p</span> ".to_string()
        );
    }

    #[test]
    fn it_should_highlight_th() {
        assert_eq!(highlight("The Cat witH a someThing"), "<span class='Th'>Th</span>e <span class='Ptk'>C</span>a<span class='Ptk'>t</span> <span class='W'>w</span>i<span class='Th'>tH</span> a some<span class='Th'>Th</span>i<span class='Ng'>ng</span>".to_string());
    }

    #[test]
    fn it_should_highlight_ch() {
        assert_eq!(highlight("Cheese, cHicken, beach"), "<span class='Ch'>Ch</span>eese, <span class='Ch'>cH</span>icken, bea<span class='Ch'>ch</span>".to_string());
    }

    #[test]
    fn it_should_highlight_w() {
        assert_eq!(
            highlight("What, where, toward"),
            "<span class='W'>W</span>ha<span class='Ptk'>t</span>, <span class='W'>w</span>here, <span class='Ptk'>t</span>oward".to_string()
        );
    }

    #[test]
    fn it_should_highlight_v() {
        assert_eq!(highlight("Vote, vital, viva"), "<span class='V'>V</span>ote, <span class='V'>v</span>ital, <span class='V'>v</span>iva".to_string());
    }

    #[test]
    fn it_should_highlight_ng() {
        assert_eq!(
            highlight("Going, nginx"),
            "Goi<span class='Ng'>ng</span>, <span class='Ng'>ng</span>inx".to_string()
        );
    }

    #[test]
    fn it_should_highlight_dj() {
        assert_eq!(
            highlight("John, just, enjoy"),
            "<span class='Dj'>J</span>ohn, <span class='Dj'>j</span>us<span class='Ptk'>t</span>, enjoy".to_string()
        );
    }

    #[test]
    fn it_should_highlight_with_non_breakable_chars() {
        assert_eq!(
            highlight("Put\u{a0}W"),
            "<span class='Ptk'>P</span>u<span class='Ptk'>t</span>\u{a0}<span class='W'>W</span>"
                .to_string()
        );
    }

    #[test]
    fn it_should_highlight_with_punctuation_chars() {
        assert_eq!(
            highlight("what!the such-exp:the going?Jhon much; Going."),
            "<span class='W'>w</span>ha<span class='Ptk'>t</span>!<span class='Th'>th</span>e su<span class='Ch'>ch</span>-ex<span class='Ptk'>p</span>:<span class='Th'>th</span>e goi<span class='Ng'>ng</span>?<span class='Dj'>J</span>hon mu<span class='Ch'>ch</span>; Goi<span class='Ng'>ng</span>."
                .to_string()
        );
    }
}
