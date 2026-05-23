# pizza-analysis-turkish

Turkish language analysis with apostrophe handling, Turkish-specific lowercasing (dotted/dotless I), stemming, and stop words.

Part of the [Pizza](https://pizza.rs) search engine.

## Components

| Name | Type | Description |
|------|------|-------------|
| `turkish_apostrophe` | Token Filter | Strips suffixes after apostrophes in Turkish proper nouns |
| `turkish_lowercase` | Token Filter | Turkish-specific lowercasing (İ→i, I→ı) per Turkish locale rules |
| `turkish_stem` | Token Filter | Turkish light stemmer |
| `turkish_stop` | Token Filter | Turkish stop words filter (209 words) |
| `turkish` | Analyzer | Full pipeline: apostrophe → turkish_lowercase → stop → stem |

## Usage

### Built-in Analyzer

```json
{
  "analyzer": {
    "type": "turkish"
  }
}
```

### Custom Pipeline

```json
{
  "analyzer": {
    "type": "custom",
    "tokenizer": "standard",
    "filter": ["turkish_apostrophe", "turkish_lowercase", "turkish_stem", "turkish_stop"]
  }
}
```

## License

MIT — see [LICENSE](LICENSE).

## Related Crates

- [analysis-core](https://github.com/pizza-rs/analysis-core) — Core analysis components and pipeline
- [analysis-icu](https://github.com/pizza-rs/analysis-icu) — ICU Unicode normalization and tokenization
- [analysis-english](https://github.com/pizza-rs/analysis-english) — English analysis
- [analysis-all](https://github.com/pizza-rs/analysis-all) — Meta-crate registering all analyzers
