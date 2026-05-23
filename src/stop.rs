//! Turkish stop words (from Lucene/Snowball project).

use alloc::borrow::Cow;
use alloc::vec::Vec;
use hashbrown::HashSet;
use once_cell::sync::Lazy;
use pizza_engine::analysis::{Token, TokenFilter};

/// Default Turkish stop words sourced from Apache Lucene.
static DEFAULT_STOP_WORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let words: &[&str] = &[
    "acaba",
    "altmış",
    "altı",
    "ama",
    "ancak",
    "arada",
    "aslında",
    "ayrıca",
    "bana",
    "bazı",
    "belki",
    "ben",
    "benden",
    "beni",
    "benim",
    "beri",
    "beş",
    "bile",
    "bin",
    "bir",
    "biri",
    "birkaç",
    "birkez",
    "birçok",
    "birşey",
    "birşeyi",
    "biz",
    "bizden",
    "bize",
    "bizi",
    "bizim",
    "bu",
    "buna",
    "bunda",
    "bundan",
    "bunlar",
    "bunları",
    "bunların",
    "bunu",
    "bunun",
    "burada",
    "böyle",
    "böylece",
    "da",
    "daha",
    "dahi",
    "de",
    "defa",
    "değil",
    "diye",
    "diğer",
    "doksan",
    "dokuz",
    "dolayı",
    "dolayısıyla",
    "dört",
    "edecek",
    "eden",
    "ederek",
    "edilecek",
    "ediliyor",
    "edilmesi",
    "ediyor",
    "elli",
    "en",
    "etmesi",
    "etti",
    "ettiği",
    "ettiğini",
    "eğer",
    "gibi",
    "göre",
    "halen",
    "hangi",
    "hatta",
    "hem",
    "henüz",
    "hep",
    "hepsi",
    "her",
    "herhangi",
    "herkesin",
    "hiç",
    "hiçbir",
    "iki",
    "ile",
    "ilgili",
    "ise",
    "itibaren",
    "itibariyle",
    "için",
    "işte",
    "kadar",
    "karşın",
    "katrilyon",
    "kendi",
    "kendilerine",
    "kendini",
    "kendisi",
    "kendisine",
    "kendisini",
    "kez",
    "ki",
    "kim",
    "kimden",
    "kime",
    "kimi",
    "kimse",
    "kırk",
    "milyar",
    "milyon",
    "mu",
    "mü",
    "mı",
    "nasıl",
    "ne",
    "neden",
    "nedenle",
    "nerde",
    "nerede",
    "nereye",
    "niye",
    "niçin",
    "o",
    "olan",
    "olarak",
    "oldu",
    "olduklarını",
    "olduğu",
    "olduğunu",
    "olmadı",
    "olmadığı",
    "olmak",
    "olması",
    "olmayan",
    "olmaz",
    "olsa",
    "olsun",
    "olup",
    "olur",
    "olursa",
    "oluyor",
    "on",
    "ona",
    "ondan",
    "onlar",
    "onlardan",
    "onları",
    "onların",
    "onu",
    "onun",
    "otuz",
    "oysa",
    "pek",
    "rağmen",
    "sadece",
    "sanki",
    "sekiz",
    "seksen",
    "sen",
    "senden",
    "seni",
    "senin",
    "siz",
    "sizden",
    "sizi",
    "sizin",
    "tarafından",
    "trilyon",
    "tüm",
    "var",
    "vardı",
    "ve",
    "veya",
    "ya",
    "yani",
    "yapacak",
    "yapmak",
    "yaptı",
    "yaptıkları",
    "yaptığı",
    "yaptığını",
    "yapılan",
    "yapılması",
    "yapıyor",
    "yedi",
    "yerine",
    "yetmiş",
    "yine",
    "yirmi",
    "yoksa",
    "yüz",
    "zaten",
    "çok",
    "çünkü",
    "öyle",
    "üzere",
    "üç",
    "şey",
    "şeyden",
    "şeyi",
    "şeyler",
    "şu",
    "şuna",
    "şunda",
    "şundan",
    "şunları",
    "şunu",
    "şöyle",
    ];
    words.iter().copied().collect()
});

/// Removes Turkish stop words from the token stream.
#[derive(Clone, Debug)]
pub struct TurkishStopFilter {
    stop_words: HashSet<String>,
}

impl Default for TurkishStopFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl TurkishStopFilter {
    pub fn new() -> Self {
        Self {
            stop_words: DEFAULT_STOP_WORDS.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn with_words(words: &[&str]) -> Self {
        Self {
            stop_words: words.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl TokenFilter for TurkishStopFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let term = token.term.as_ref();
        if self.stop_words.contains(term) {
            return (true, None);
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_word_count() {
        assert!(DEFAULT_STOP_WORDS.len() >= 209);
    }

    #[test]
    fn test_filters_stop_word() {
        let f = TurkishStopFilter::new();
        let word = DEFAULT_STOP_WORDS.iter().next().unwrap();
        let mut token = Token::new(word, 0, word.len() as u32, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }

    #[test]
    fn test_passes_non_stop_word() {
        let f = TurkishStopFilter::new();
        let mut token = Token::new("xyzzy_not_a_stop_word", 0, 21, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted);
    }

    #[test]
    fn test_custom_words() {
        let f = TurkishStopFilter::with_words(&["custom", "words"]);
        let mut token = Token::new("custom", 0, 6, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }
}
