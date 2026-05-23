//! Turkish-specific lowercase handling.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::{Token, TokenFilter};

/// Turkish locale-aware lowercase filter.
///
/// Handles the dotted/dotless I distinction:
/// - İ (U+0130) → i
/// - I (U+0049) → ı (U+0131) (NOT regular 'i')
/// - i → i (unchanged)
/// - ı → ı (unchanged)
#[derive(Clone, Debug, Default)]
pub struct TurkishLowercaseFilter;

impl TurkishLowercaseFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for TurkishLowercaseFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        if text.is_empty() {
            return (false, None);
        }

        // Check if any conversion is needed
        let needs_change = text.chars().any(|c| c.is_uppercase() || c == '\u{0130}');
        if !needs_change {
            return (false, None);
        }

        let mut result = String::with_capacity(text.len());
        for c in text.chars() {
            match c {
                '\u{0130}' => result.push('i'),         // İ → i
                'I' => result.push('\u{0131}'),          // I → ı (dotless)
                _ => {
                    for lc in c.to_lowercase() {
                        result.push(lc);
                    }
                }
            }
        }

        token.term = Cow::Owned(result);
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dotted_i() {
        let f = TurkishLowercaseFilter::new();
        // İstanbul → istanbul
        let mut token = Token::new("\u{0130}stanbul", 0, 9, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "istanbul");
    }

    #[test]
    fn test_dotless_i() {
        let f = TurkishLowercaseFilter::new();
        // ISLAK → ıslak (not "islak")
        let mut token = Token::new("ISLAK", 0, 5, 0);
        f.filter(&mut token);
        assert!(token.term.starts_with('\u{0131}'));
    }

    #[test]
    fn test_already_lowercase() {
        let f = TurkishLowercaseFilter::new();
        let mut token = Token::new("merhaba", 0, 7, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "merhaba");
    }
}
