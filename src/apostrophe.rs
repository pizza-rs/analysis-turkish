//! Turkish apostrophe filter.
//!
//! Strips suffixes after an apostrophe character — common in Turkish
//! where proper nouns carry inflectional suffixes separated by an apostrophe.
//! e.g. "İstanbul'un" → "İstanbul"

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::{Token, TokenFilter};

/// Removes everything at and after an apostrophe in Turkish text.
///
/// Equivalent to Lucene's `ApostropheFilter`.
#[derive(Clone, Debug, Default)]
pub struct TurkishApostropheFilter;

impl TurkishApostropheFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for TurkishApostropheFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        // Look for apostrophe characters (ASCII ' and Unicode right single quote ')
        if let Some(pos) = text.find(|c: char| c == '\'' || c == '\u{2019}') {
            if pos == 0 {
                return (true, None); // entire token is apostrophe
            }
            let truncated: String = text[..pos].to_string();
            token.term = Cow::Owned(truncated);
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apostrophe_suffix_removal() {
        let f = TurkishApostropheFilter::new();
        let mut token = Token::new("İstanbul'un", 0, 14, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "İstanbul");
    }

    #[test]
    fn test_unicode_apostrophe() {
        let f = TurkishApostropheFilter::new();
        let mut token = Token::new("Ankara\u{2019}da", 0, 11, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "Ankara");
    }

    #[test]
    fn test_no_apostrophe() {
        let f = TurkishApostropheFilter::new();
        let mut token = Token::new("merhaba", 0, 7, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "merhaba");
    }
}
