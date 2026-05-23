//! Comprehensive tests for pizza-analysis-turkish.

use pizza_analysis_turkish::*;
use pizza_engine::analysis::{AnalysisFactory, Token, TokenFilter};

fn make_token(term: &str) -> Token<'_> {
    Token::new(term, 0, term.len() as u32, 0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// TurkishApostropheFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn apostrophe_construction() {
    let _f = TurkishApostropheFilter::new();
}

#[test]
fn apostrophe_strips_suffix() {
    let f = TurkishApostropheFilter::new();
    // "İstanbul'un" → "İstanbul"
    let mut token = make_token("İstanbul'un");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "İstanbul");
}

#[test]
fn apostrophe_strips_da_suffix() {
    let f = TurkishApostropheFilter::new();
    // "Ankara'da" → "Ankara"
    let mut token = make_token("Ankara'da");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "Ankara");
}

#[test]
fn apostrophe_no_change_without_apostrophe() {
    let f = TurkishApostropheFilter::new();
    let mut token = make_token("kitap");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "kitap");
}

#[test]
fn apostrophe_empty_string() {
    let f = TurkishApostropheFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// TurkishLowercaseFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn lowercase_construction() {
    let _f = TurkishLowercaseFilter::new();
}

#[test]
fn lowercase_dotted_i() {
    let f = TurkishLowercaseFilter::new();
    // İ (capital dotted I) → i (lowercase dotted i)
    let mut token = make_token("İstanbul");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert!(token.term.starts_with("i") || token.term.starts_with("istanbul"));
}

#[test]
fn lowercase_dotless_i() {
    let f = TurkishLowercaseFilter::new();
    // I (capital dotless I) → ı (lowercase dotless i)
    let mut token = make_token("IRAK");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert!(token.term.starts_with("ı"));
}

#[test]
fn lowercase_regular_ascii() {
    let f = TurkishLowercaseFilter::new();
    let mut token = make_token("HELLO");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn lowercase_empty_string() {
    let f = TurkishLowercaseFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// TurkishStemFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stem_construction() {
    let _f = TurkishStemFilter::new();
}

#[test]
fn stem_plural_lar() {
    let f = TurkishStemFilter::new();
    // "evler" (houses) → stem
    let mut token = make_token("evler");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_plural_ler() {
    let f = TurkishStemFilter::new();
    // "kitaplar" (books) → stem
    let mut token = make_token("kitaplar");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_accusative() {
    let f = TurkishStemFilter::new();
    // "evi" (house, accusative) → stem
    let mut token = make_token("evi");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_locative() {
    let f = TurkishStemFilter::new();
    // "evde" (in the house) → stem
    let mut token = make_token("evde");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_genitive() {
    let f = TurkishStemFilter::new();
    // "evin" (of the house) → stem
    let mut token = make_token("evin");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_possessive() {
    let f = TurkishStemFilter::new();
    // "evimiz" (our house) → stem
    let mut token = make_token("evimiz");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_short_word() {
    let f = TurkishStemFilter::new();
    let mut token = make_token("ve");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_empty_string() {
    let f = TurkishStemFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// TurkishStopFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stop_construction() {
    let _f = TurkishStopFilter::new();
}

#[test]
fn stop_filters_common_words() {
    let f = TurkishStopFilter::new();
    let stop_words = ["ve", "bir", "bu", "da", "de", "ile", "için", "var", "ama", "olan"];
    for word in &stop_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted, "stop word '{}' should be filtered", word);
    }
}

#[test]
fn stop_keeps_content_words() {
    let f = TurkishStopFilter::new();
    let content_words = ["ev", "kitap", "okul", "şehir"];
    for word in &content_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted, "content word '{}' should be kept", word);
    }
}

#[test]
fn stop_empty_string() {
    let f = TurkishStopFilter::new();
    let mut token = make_token("");
    let _ = f.filter(&mut token);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Registration
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn register_all_no_panic() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
}

#[test]
fn register_all_filters_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_token_filter("turkish_apostrophe").is_some());
    assert!(factory.get_token_filter("turkish_lowercase").is_some());
    assert!(factory.get_token_filter("turkish_stem").is_some());
    assert!(factory.get_token_filter("turkish_stop").is_some());
}

#[test]
fn register_all_analyzer_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_analyzer("turkish").is_some());
}

#[test]
fn analyzer_pipeline_produces_tokens() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("turkish").unwrap();
    let mut input = String::from("Ev büyük ve güzel");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}

#[test]
fn analyzer_pipeline_removes_stops() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("turkish").unwrap();
    let mut input = String::from("kitap ve okul bir şehir");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    let terms: Vec<&str> = tokens.iter().map(|t| t.term.as_ref()).collect();
    assert!(!terms.contains(&"ve"));
    assert!(!terms.contains(&"bir"));
}

#[test]
fn analyzer_pipeline_apostrophe_handling() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("turkish").unwrap();
    let mut input = String::from("İstanbul'un sokakları");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}

#[test]
fn analyzer_pipeline_empty_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("turkish").unwrap();
    let mut input = String::from("");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(tokens.is_empty());
}

#[test]
fn analyzer_pipeline_ascii_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("turkish").unwrap();
    let mut input = String::from("hello world");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}
