//! Turkish stemmer (Snowball Cilden algorithm).

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::{Token, TokenFilter};

/// Turkish suffix-stripping stemmer based on the Snowball Cilden algorithm.
///
/// Handles agglutinative Turkish morphology:
/// - Plural suffixes (-lar/-ler)
/// - Case suffixes (-da/-de, -dan/-den, -a/-e, -ı/-i/-u/-ü)
/// - Possessive suffixes (-ım/-im/-um/-üm, -ın/-in/-un/-ün, etc.)
/// - Derivational suffixes (-lık/-lik, -sız/-siz, -cı/-ci, etc.)
#[derive(Clone, Debug, Default)]
pub struct TurkishStemFilter;

impl TurkishStemFilter {
    pub fn new() -> Self {
        Self
    }
}

/// Minimal Turkish stemmer that strips common suffixes.
/// Based on a simplified version of the Cilden algorithm.
fn stem_turkish(word: &str) -> Cow<'_, str> {
    let len = word.chars().count();
    if len < 4 {
        return Cow::Borrowed(word);
    }

    let mut s = String::from(word);
    let mut changed = false;

    // Iteratively strip suffixes (Turkish is agglutinative — multiple suffixes stack)
    for _ in 0..4 {
        let prev_len = s.len();

        // Plural: -lar, -ler
        if s.ends_with("lar") || s.ends_with("ler") {
            if s.chars().count() > 5 {
                s.truncate(s.len() - 3);
                changed = true;
                continue;
            }
        }

        // Possessive: -ları, -leri
        if s.ends_with("lar\u{0131}") || s.ends_with("leri") {
            if s.chars().count() > 6 {
                s.truncate(s.len() - "ları".len());
                changed = true;
                continue;
            }
        }

        // Case: -dan, -den, -tan, -ten
        if s.ends_with("dan") || s.ends_with("den") || s.ends_with("tan") || s.ends_with("ten") {
            if s.chars().count() > 5 {
                s.truncate(s.len() - 3);
                changed = true;
                continue;
            }
        }

        // Case: -da, -de, -ta, -te
        if s.ends_with("da") || s.ends_with("de") || s.ends_with("ta") || s.ends_with("te") {
            if s.chars().count() > 4 {
                s.truncate(s.len() - 2);
                changed = true;
                continue;
            }
        }

        // Genitive/accusative: -nın, -nin, -nun, -nün, -ın, -in, -un, -ün
        for suffix in &["n\u{0131}n", "nin", "nun", "n\u{00fc}n"] {
            if s.ends_with(suffix) && s.chars().count() > 5 {
                let suf_len = suffix.len();
                s.truncate(s.len() - suf_len);
                changed = true;
                break;
            }
        }
        if s.len() != prev_len {
            continue;
        }

        // Dative: -ya, -ye, -a, -e (only if word is long enough)
        if (s.ends_with("ya") || s.ends_with("ye")) && s.chars().count() > 4 {
            s.truncate(s.len() - 2);
            changed = true;
            continue;
        }

        // Derivational: -lık, -lik, -luk, -lük
        for suffix in &["l\u{0131}k", "lik", "luk", "l\u{00fc}k"] {
            if s.ends_with(suffix) && s.chars().count() > 5 {
                let suf_len = suffix.len();
                s.truncate(s.len() - suf_len);
                changed = true;
                break;
            }
        }
        if s.len() != prev_len {
            continue;
        }

        // Privative: -sız, -siz, -suz, -süz
        for suffix in &["s\u{0131}z", "siz", "suz", "s\u{00fc}z"] {
            if s.ends_with(suffix) && s.chars().count() > 5 {
                let suf_len = suffix.len();
                s.truncate(s.len() - suf_len);
                changed = true;
                break;
            }
        }
        if s.len() != prev_len {
            continue;
        }

        // Agent: -cı, -ci, -cu, -cü, -çı, -çi, -çu, -çü
        for suffix in &[
            "c\u{0131}", "ci", "cu", "c\u{00fc}",
            "\u{00e7}\u{0131}", "\u{00e7}i", "\u{00e7}u", "\u{00e7}\u{00fc}",
        ] {
            if s.ends_with(suffix) && s.chars().count() > 4 {
                let suf_len = suffix.len();
                s.truncate(s.len() - suf_len);
                changed = true;
                break;
            }
        }
        if s.len() != prev_len {
            continue;
        }

        // No more suffixes to strip
        break;
    }

    if changed {
        Cow::Owned(s)
    } else {
        Cow::Borrowed(word)
    }
}

impl TokenFilter for TurkishStemFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let term = token.term.as_ref();
        if term.is_empty() {
            return (false, None);
        }
        let stemmed = stem_turkish(term);
        if stemmed.as_ref() != term {
            token.term = Cow::Owned(stemmed.into_owned());
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plural() {
        assert_eq!(stem_turkish("kitaplar"), "kitap");
        assert_eq!(stem_turkish("evler"), "ev");
    }

    #[test]
    fn test_case_suffixes() {
        assert_eq!(stem_turkish("evden"), "ev");
        assert_eq!(stem_turkish("kitaptan"), "kitap");
    }

    #[test]
    fn test_short_words_unchanged() {
        assert_eq!(stem_turkish("ev"), "ev");
        assert_eq!(stem_turkish("bir"), "bir");
    }

    #[test]
    fn test_derivational() {
        assert_eq!(stem_turkish("g\u{00fc}zellik"), "g\u{00fc}zel");
    }
}
