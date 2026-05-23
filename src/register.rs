//! Register Turkish analysis components into [`AnalysisFactory`].

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use pizza_engine::analysis::{
    Analyzer, AnalysisFactory, Normalizer, StandardTokenizer, TokenFilter,
    Tokenizer,
};

use crate::{TurkishApostropheFilter, TurkishLowercaseFilter, TurkishStemFilter, TurkishStopFilter};

/// Register Turkish token filters and the `"turkish"` analyzer.
///
/// Note: The Turkish analyzer uses `TurkishLowercaseFilter` instead of the
/// standard `LowercaseNormalizer` to handle dotted/dotless I correctly.
pub fn register_all(factory: &mut AnalysisFactory) {
    factory.register_token_filter("turkish_apostrophe", Box::new(TurkishApostropheFilter::new()));
    factory.register_token_filter("turkish_lowercase", Box::new(TurkishLowercaseFilter::new()));
    factory.register_token_filter("turkish_stem", Box::new(TurkishStemFilter::new()));
    factory.register_token_filter("turkish_stop", Box::new(TurkishStopFilter::new()));

    let normalizers: Vec<Box<dyn Normalizer>> = vec![];
    let tokenizer: Box<dyn Tokenizer> = Box::new(StandardTokenizer::new());
    let filters: Vec<Box<dyn TokenFilter>> = vec![
        Box::new(TurkishApostropheFilter::new()),
        Box::new(TurkishLowercaseFilter::new()),
        Box::new(TurkishStopFilter::new()),
        Box::new(TurkishStemFilter::new()),
    ];
    factory.register_analyzer("turkish", Analyzer::new(normalizers, tokenizer, filters));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_all_no_panic() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
    }

    #[test]
    fn test_filters_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_token_filter("turkish_lowercase").is_some());
        assert!(factory.get_token_filter("turkish_stem").is_some());
        assert!(factory.get_token_filter("turkish_stop").is_some());
    }

    #[test]
    fn test_analyzer_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_analyzer("turkish").is_some());
    }

    #[test]
    fn test_analyzer_pipeline() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        let analyzer = factory.get_analyzer("turkish").unwrap();
        let mut input = String::from("İstanbul bir büyük şehirdir");
        let tokens = analyzer.analyze_and_return_tokens(&mut input);
        // "bir" is stop word, "İ" should be lowercased to "i"
        assert!(!tokens.iter().any(|t| t.term == "bir"));
        assert!(tokens.len() >= 2);
    }
}
