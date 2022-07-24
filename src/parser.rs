use crate::scanner::Scanner;
use crate::sound::{Sound, SoundKind};

fn add_sound_from_two_letters(
    first_letter: &char,
    second_letter: &char,
    kind: SoundKind,
    sounds: &mut Vec<Sound>,
) {
    sounds.push(Sound::new(
        kind,
        first_letter.to_string() + &String::from(*second_letter),
    ));
}

/// Parse text to sounds
///
/// ## Example
///
/// ```rust
/// use text_to_sounds::{parse, SoundKind, Sound};
///
/// let sounds = vec![
///     Sound::new(SoundKind::Th, String::from("Th")),
///     Sound::new(SoundKind::Undefined, String::from("e")),
///     Sound::new(SoundKind::Undefined, String::from(" ")),
///     Sound::new(SoundKind::Ptk, String::from("t")),
///     Sound::new(SoundKind::Undefined, String::from("e")),
///     Sound::new(SoundKind::Undefined, String::from("x")),
///     Sound::new(SoundKind::Ptk, String::from("t")),
///     Sound::new(SoundKind::Undefined, String::from(" ")),
///     Sound::new(SoundKind::Dj, String::from("j")),
///     Sound::new(SoundKind::Undefined, String::from("u")),
///     Sound::new(SoundKind::Undefined, String::from("s")),
///     Sound::new(SoundKind::Ptk, String::from("t")),
///     Sound::new(SoundKind::Undefined, String::from(" ")),
///     Sound::new(SoundKind::Undefined, String::from("i")),
///     Sound::new(SoundKind::Undefined, String::from("n")),
///     Sound::new(SoundKind::Undefined, String::from(" ")),
///     Sound::new(SoundKind::Ptk, String::from("c")),
///     Sound::new(SoundKind::Undefined, String::from("a")),
///     Sound::new(SoundKind::Undefined, String::from("s")),
///     Sound::new(SoundKind::Undefined, String::from("e")),
/// ];
///
/// assert_eq!(parse("The text just in case"), sounds);
/// ```
pub fn parse<T: AsRef<str>>(text: T) -> Vec<Sound> {
    let text = text.as_ref();

    let mut sounds = vec![];

    if text.is_empty() {
        return sounds;
    }

    let text_splited = text.split(' ').collect::<Vec<&str>>();

    for (index, word) in text_splited.iter().enumerate() {
        let mut scanner = Scanner::new(word);

        while !scanner.is_done() {
            match scanner.peek() {
                letter @ ('c' | 'C')
                    if !scanner.is_last() && scanner.is_next_any(vec!['h', 'H']) =>
                {
                    let next_letter = scanner.peek_next();

                    add_sound_from_two_letters(letter, next_letter, SoundKind::Ch, &mut sounds);

                    scanner.pop();
                    scanner.pop();
                }
                letter @ ('p' | 'P' | 't' | 'T' | 'c' | 'C')
                    if scanner.is_first() || scanner.is_last() =>
                {
                    if (letter == &'t' || letter == &'T')
                        && !scanner.is_last()
                        && scanner.is_next_any(vec!['h', 'H'])
                    {
                        let next_letter = scanner.peek_next();

                        add_sound_from_two_letters(letter, next_letter, SoundKind::Th, &mut sounds);

                        scanner.pop();
                        scanner.pop();

                        continue;
                    }

                    sounds.push(Sound::new(SoundKind::Ptk, letter.to_string()));
                    scanner.pop();
                }
                letter @ ('t' | 'T') if scanner.is_next_any(vec!['h', 'H']) => {
                    let next_letter = scanner.peek_next();

                    add_sound_from_two_letters(letter, next_letter, SoundKind::Th, &mut sounds);

                    scanner.pop();
                    scanner.pop();
                }
                letter @ ('w' | 'W') if scanner.is_first() => {
                    sounds.push(Sound::new(SoundKind::W, letter.to_string()));
                    scanner.pop();
                }
                letter @ ('v' | 'V') if scanner.is_first() => {
                    sounds.push(Sound::new(SoundKind::V, letter.to_string()));
                    scanner.pop();
                }
                letter @ ('n' | 'N')
                    if !scanner.is_last() && scanner.is_next_any(vec!['g', 'G', 'k', 'K']) =>
                {
                    let next_letter = scanner.peek_next();

                    add_sound_from_two_letters(letter, next_letter, SoundKind::Ng, &mut sounds);

                    scanner.pop();
                    scanner.pop();
                }
                letter @ ('j' | 'J') if scanner.is_first() => {
                    sounds.push(Sound::new(SoundKind::Dj, letter.to_string()));
                    scanner.pop();
                }
                _ => {
                    let letter = scanner.pop();
                    sounds.push(Sound::new(SoundKind::Undefined, letter.to_string()));
                }
            }
        }

        if index + 1 != text_splited.len() {
            sounds.push(Sound::new(SoundKind::Undefined, " ".to_string()));
        }
    }

    sounds
}

#[cfg(test)]
mod parse {
    use super::{parse, Sound, SoundKind};

    #[test]
    fn it_should_parse_empty() {
        assert_eq!(parse(""), Vec::<Sound>::new());
    }

    #[test]
    fn it_should_parse_space() {
        let sounds = vec![Sound::new(SoundKind::Undefined, String::from(" "))];

        assert_eq!(parse(" "), sounds);
    }

    #[test]
    fn it_should_parse_p_and_t() {
        let sounds = vec![
            Sound::new(SoundKind::Ptk, String::from("p")),
            Sound::new(SoundKind::Undefined, String::from("u")),
            Sound::new(SoundKind::Ptk, String::from("t")),
        ];

        assert_eq!(parse("put"), sounds);
    }

    #[test]
    fn it_should_parse_th_in_the_beginning() {
        let sounds = vec![
            Sound::new(SoundKind::Th, String::from("th")),
            Sound::new(SoundKind::Undefined, String::from("e")),
        ];

        assert_eq!(parse("the"), sounds);
    }

    #[test]
    fn it_should_parse_th_in_the_middle() {
        let sounds = vec![
            Sound::new(SoundKind::Ptk, String::from("t")),
            Sound::new(SoundKind::Undefined, String::from("o")),
            Sound::new(SoundKind::Undefined, String::from("g")),
            Sound::new(SoundKind::Undefined, String::from("e")),
            Sound::new(SoundKind::Th, String::from("th")),
            Sound::new(SoundKind::Undefined, String::from("e")),
            Sound::new(SoundKind::Undefined, String::from("r")),
        ];

        assert_eq!(parse("together"), sounds);
    }

    #[test]
    fn it_should_parse_th_and_t() {
        let sounds = vec![
            Sound::new(SoundKind::Th, String::from("th")),
            Sound::new(SoundKind::Undefined, String::from("r")),
            Sound::new(SoundKind::Undefined, String::from("o")),
            Sound::new(SoundKind::Undefined, String::from("t")),
            Sound::new(SoundKind::Undefined, String::from("t")),
            Sound::new(SoundKind::Undefined, String::from("l")),
            Sound::new(SoundKind::Undefined, String::from("e")),
        ];

        assert_eq!(parse("throttle"), sounds);
    }

    #[test]
    fn it_should_parse_c_and_t() {
        let sounds = vec![
            Sound::new(SoundKind::Ptk, String::from("c")),
            Sound::new(SoundKind::Undefined, String::from("a")),
            Sound::new(SoundKind::Ptk, String::from("t")),
        ];

        assert_eq!(parse("cat"), sounds);
    }

    #[test]
    fn it_should_parse_uppercase_c_and_t() {
        let sounds = vec![
            Sound::new(SoundKind::Ptk, String::from("C")),
            Sound::new(SoundKind::Undefined, String::from("a")),
            Sound::new(SoundKind::Ptk, String::from("t")),
        ];

        assert_eq!(parse("Cat"), sounds);
    }

    #[test]
    fn it_should_parse_text_with_space() {
        let sounds = vec![
            Sound::new(SoundKind::Ptk, String::from("p")),
            Sound::new(SoundKind::Undefined, String::from("u")),
            Sound::new(SoundKind::Ptk, String::from("t")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::Ptk, String::from("t")),
            Sound::new(SoundKind::Undefined, String::from("o")),
            Sound::new(SoundKind::Undefined, String::from("g")),
            Sound::new(SoundKind::Undefined, String::from("e")),
            Sound::new(SoundKind::Th, String::from("th")),
            Sound::new(SoundKind::Undefined, String::from("e")),
            Sound::new(SoundKind::Undefined, String::from("r")),
        ];

        assert_eq!(parse("put together"), sounds);
    }

    #[test]
    fn it_should_parse_long_sentence_with_different_registrs() {
        let sounds = vec![
            Sound::new(SoundKind::Th, String::from("Th")),
            Sound::new(SoundKind::Undefined, String::from("e")),
            Sound::new(SoundKind::Undefined, String::from("n")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::Ptk, String::from("P")),
            Sound::new(SoundKind::Undefined, String::from("u")),
            Sound::new(SoundKind::Ptk, String::from("T")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::Ptk, String::from("t")),
            Sound::new(SoundKind::Undefined, String::from("O")),
            Sound::new(SoundKind::Undefined, String::from("g")),
            Sound::new(SoundKind::Undefined, String::from("E")),
            Sound::new(SoundKind::Th, String::from("TH")),
            Sound::new(SoundKind::Undefined, String::from("e")),
            Sound::new(SoundKind::Undefined, String::from("r")),
        ];

        assert_eq!(parse("Then PuT tOgETHer"), sounds);
    }

    #[test]
    fn it_should_parse_dj() {
        let sounds = vec![
            Sound::new(SoundKind::Dj, String::from("J")),
            Sound::new(SoundKind::Undefined, String::from("o")),
            Sound::new(SoundKind::Undefined, String::from("h")),
            Sound::new(SoundKind::Undefined, String::from("n")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::Undefined, String::from("g")),
            Sound::new(SoundKind::Undefined, String::from("o")),
            Sound::new(SoundKind::Ptk, String::from("t")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::Dj, String::from("j")),
            Sound::new(SoundKind::Undefined, String::from("o")),
            Sound::new(SoundKind::Undefined, String::from("b")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::Undefined, String::from("i")),
            Sound::new(SoundKind::Undefined, String::from("n")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::Dj, String::from("J")),
            Sound::new(SoundKind::Undefined, String::from("a")),
            Sound::new(SoundKind::Undefined, String::from("n")),
            Sound::new(SoundKind::Undefined, String::from("u")),
            Sound::new(SoundKind::Undefined, String::from("a")),
            Sound::new(SoundKind::Undefined, String::from("r")),
            Sound::new(SoundKind::Undefined, String::from("y")),
        ];

        assert_eq!(parse("John got job in January"), sounds);
    }

    #[test]
    fn it_should_parse_ch() {
        let sounds = vec![
            Sound::new(SoundKind::Undefined, String::from("S")),
            Sound::new(SoundKind::Undefined, String::from("u")),
            Sound::new(SoundKind::Ch, String::from("ch")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::Ch, String::from("CH")),
            Sound::new(SoundKind::Undefined, String::from("o")),
            Sound::new(SoundKind::Undefined, String::from("o")),
            Sound::new(SoundKind::Undefined, String::from("s")),
            Sound::new(SoundKind::Undefined, String::from("e")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::W, String::from("w")),
            Sound::new(SoundKind::Undefined, String::from("h")),
            Sound::new(SoundKind::Undefined, String::from("i")),
            Sound::new(SoundKind::Ch, String::from("Ch")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::Ch, String::from("cH")),
            Sound::new(SoundKind::Undefined, String::from("e")),
            Sound::new(SoundKind::Undefined, String::from("a")),
            Sound::new(SoundKind::Ptk, String::from("p")),
        ];

        assert_eq!(parse("Such CHoose whiCh cHeap"), sounds);
    }

    #[test]
    fn it_should_parse_w_and_v() {
        let sounds = vec![
            Sound::new(SoundKind::W, String::from("W")),
            Sound::new(SoundKind::Undefined, String::from("h")),
            Sound::new(SoundKind::Undefined, String::from("a")),
            Sound::new(SoundKind::Ptk, String::from("t")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::Undefined, String::from("i")),
            Sound::new(SoundKind::Undefined, String::from("s")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::V, String::from("v")),
            Sound::new(SoundKind::Undefined, String::from("e")),
            Sound::new(SoundKind::Ptk, String::from("t")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::Undefined, String::from("a")),
            Sound::new(SoundKind::Undefined, String::from("n")),
            Sound::new(SoundKind::Undefined, String::from("d")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::W, String::from("w")),
            Sound::new(SoundKind::Undefined, String::from("e")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::W, String::from("w")),
            Sound::new(SoundKind::Undefined, String::from("i")),
            Sound::new(SoundKind::Undefined, String::from("l")),
            Sound::new(SoundKind::Undefined, String::from("l")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::V, String::from("V")),
            Sound::new(SoundKind::Undefined, String::from("i")),
            Sound::new(SoundKind::Undefined, String::from("e")),
            Sound::new(SoundKind::Undefined, String::from("w")),
        ];

        assert_eq!(parse("What is vet and we will View"), sounds);
    }

    #[test]
    fn it_should_parse_ng() {
        let sounds = vec![
            Sound::new(SoundKind::Ptk, String::from("P")),
            Sound::new(SoundKind::Undefined, String::from("i")),
            Sound::new(SoundKind::Ng, String::from("nK")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::Undefined, String::from("b")),
            Sound::new(SoundKind::Undefined, String::from("r")),
            Sound::new(SoundKind::Undefined, String::from("i")),
            Sound::new(SoundKind::Ng, String::from("Ng")),
            Sound::new(SoundKind::Undefined, String::from("i")),
            Sound::new(SoundKind::Ng, String::from("ng")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::Undefined, String::from("s")),
            Sound::new(SoundKind::Undefined, String::from("o")),
            Sound::new(SoundKind::Undefined, String::from("m")),
            Sound::new(SoundKind::Undefined, String::from("e")),
            Sound::new(SoundKind::Th, String::from("th")),
            Sound::new(SoundKind::Undefined, String::from("i")),
            Sound::new(SoundKind::Ng, String::from("ng")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::Ptk, String::from("t")),
            Sound::new(SoundKind::Undefined, String::from("o")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::Undefined, String::from("K")),
            Sound::new(SoundKind::Undefined, String::from("i")),
            Sound::new(SoundKind::Ng, String::from("NG")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::Ptk, String::from("t")),
            Sound::new(SoundKind::Undefined, String::from("o")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::Undefined, String::from("d")),
            Sound::new(SoundKind::Undefined, String::from("r")),
            Sound::new(SoundKind::Undefined, String::from("i")),
            Sound::new(SoundKind::Ng, String::from("Nk")),
        ];

        assert_eq!(parse("PinK briNging something to KiNG to driNk"), sounds);
    }
}
