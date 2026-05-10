# gemini-cost

[![crates.io](https://img.shields.io/crates/v/gemini-cost.svg)](https://crates.io/crates/gemini-cost)
[![docs.rs](https://img.shields.io/docsrs/gemini-cost)](https://docs.rs/gemini-cost)
[![license: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_or_Apache--2.0-blue.svg)](#license)

Calculate Google Gemini API call cost from a usage block. Cache-aware,
two-tier pricing for >200k-token prompts, supports the Gemini 2.5 and 2.0
families. Bring your own pricing override. Zero runtime dependencies.

## Why

Gemini's billing has a wrinkle: 2.5 Pro charges roughly **2x** when the
prompt exceeds 200k tokens. Every usage logger I've seen gets this wrong
or doesn't model it at all. This crate models the two tiers and picks the
right one automatically.

## Usage

```rust
use gemini_cost::{Usage, default_pricing};

let pricing = default_pricing("gemini-2.5-pro").unwrap();

// promptTokenCount on the wire INCLUDES cached content tokens:
let usage = Usage::from_gemini(
    /* prompt_token_count          = */ 1000,
    /* candidates_token_count      = */  500,
    /* cached_content_token_count  = */  300,
);
let cost_usd = pricing.cost_for(&usage);
```

## Long-prompt tier

```rust
use gemini_cost::{default_pricing, Usage, LONG_PROMPT_THRESHOLD};

let p = default_pricing("gemini-2.5-pro").unwrap();
let short = p.cost_for(&Usage { input_tokens: 100_000, output_tokens: 0, cached_input_tokens: 0 });
let long  = p.cost_for(&Usage { input_tokens: 300_000, output_tokens: 0, cached_input_tokens: 0 });
assert!(long > short * 3.0); // ~6x at 3x tokens because of the tier flip
assert_eq!(LONG_PROMPT_THRESHOLD, 200_000);
```

## Model id normalization

Pass any Vertex resource path or dated preview snapshot; the lookup
strips the qualifier back to the base alias.

```rust
use gemini_cost::default_pricing;
assert!(default_pricing("projects/p/locations/us-central1/publishers/google/models/gemini-2.5-pro").is_some());
assert!(default_pricing("gemini-2.5-pro-preview-05-06").is_some());
```

## BYO pricing

```rust
use gemini_cost::{Pricing, Usage};
let custom = Pricing::flat(0.5, 2.0, 0.05);
let _ = custom.cost_for(&Usage::default());
```

## Pricing notes

All rates are USD per 1,000,000 tokens. Pricing is dated as of 2026-Q2.
**Verify against <https://ai.google.dev/gemini-api/docs/pricing> before
billing.**

## Features

- `serde` — derive `Serialize`/`Deserialize` on `Usage`.

```toml
[dependencies]
gemini-cost = { version = "0.1", features = ["serde"] }
```

## License

Licensed under either of [MIT](LICENSE-MIT) or
[Apache-2.0](LICENSE-APACHE) at your option.
