<div align="center">

# 🇹🇷 pizza-analysis-turkish

**Turkish text analysis plugin for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--turkish-blue)](https://github.com/pizza-rs/analysis-turkish)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

Turkish language analysis with apostrophe handling, Turkish-locale lowercasing,
stemming, and stop words. Correctly handles the Turkish İ/I/ı/i casing rules
that differ from standard Unicode case mapping.

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| TokenFilter | `turkish_apostrophe` | Strip text after apostrophe (proper noun suffixes) |
| TokenFilter | `turkish_lowercase` | Turkish-locale lowercase (I→ı, İ→i) |
| TokenFilter | `turkish_stem` | Turkish stemmer (suffix stripping) |
| TokenFilter | `turkish_stop` | Turkish stop words (209 entries) |
| Analyzer | `turkish` | Full pipeline: turkish_lowercase → apostrophe → stem → stop |

### Turkish Casing

Standard `toLowercase` maps `I` → `i`, but Turkish requires `I` → `ı` (dotless i):

| Input | Turkish | Standard |
|:------|:--------|:---------|
| I | ı | i |
| İ | i | i̇ |

### Apostrophe Handling

Turkish appends suffixes to proper nouns with an apostrophe. The filter strips
the suffix: `İstanbul'da` → `İstanbul`

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_turkish::register_all(&mut factory);

let analyzer = factory.get_analyzer("turkish").unwrap();
// "İstanbul'daki" → ["istanbul"]
```

## Installation

```toml
[dependencies]
pizza-analysis-turkish = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["turkish"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
