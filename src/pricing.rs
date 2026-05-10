//! Per-model price table.
//!
//! Some Gemini models charge a higher rate when the input prompt exceeds
//! 200,000 tokens. The pricing table carries both tiers; `cost_for`
//! picks the right one automatically.

use crate::normalize::normalize_model_id;
use crate::usage::Usage;

/// Models above this fresh-input-token count are billed at the
/// long-prompt tier.
pub const LONG_PROMPT_THRESHOLD: u64 = 200_000;

/// Per-model rates, USD per 1M tokens.
///
/// The `*_long_per_mtok` fields cover the >200k-token tier; if a model
/// has no two-tier pricing, the short and long values are equal.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pricing {
    /// Fresh input rate at <=200k tokens.
    pub input_per_mtok: f64,
    /// Output rate at <=200k tokens.
    pub output_per_mtok: f64,
    /// Cached-input rate at <=200k tokens.
    pub cached_input_per_mtok: f64,
    /// Fresh input rate at >200k tokens.
    pub input_long_per_mtok: f64,
    /// Output rate at >200k tokens.
    pub output_long_per_mtok: f64,
    /// Cached-input rate at >200k tokens.
    pub cached_input_long_per_mtok: f64,
}

impl Pricing {
    /// Build a flat single-tier pricing (no >200k uplift).
    pub fn flat(input: f64, output: f64, cached_input: f64) -> Self {
        Self {
            input_per_mtok: input,
            output_per_mtok: output,
            cached_input_per_mtok: cached_input,
            input_long_per_mtok: input,
            output_long_per_mtok: output,
            cached_input_long_per_mtok: cached_input,
        }
    }

    /// Compute USD cost for the given usage.
    pub fn cost_for(&self, usage: &Usage) -> f64 {
        let long = usage.input_tokens > LONG_PROMPT_THRESHOLD;
        let (input_r, output_r, cached_r) = if long {
            (
                self.input_long_per_mtok,
                self.output_long_per_mtok,
                self.cached_input_long_per_mtok,
            )
        } else {
            (
                self.input_per_mtok,
                self.output_per_mtok,
                self.cached_input_per_mtok,
            )
        };
        (usage.input_tokens as f64 * input_r
            + usage.output_tokens as f64 * output_r
            + usage.cached_input_tokens as f64 * cached_r)
            / 1_000_000.0
    }
}

/// Built-in pricing table. Source: ai.google.dev/gemini-api/docs/pricing
/// as of 2026-Q2. VERIFY before billing.
pub const DEFAULT_PRICING_TABLE: &[(&str, Pricing)] = &[
    (
        "gemini-2.5-pro",
        Pricing {
            input_per_mtok: 1.25,
            output_per_mtok: 10.0,
            cached_input_per_mtok: 0.3125,
            input_long_per_mtok: 2.5,
            output_long_per_mtok: 15.0,
            cached_input_long_per_mtok: 0.625,
        },
    ),
    (
        "gemini-2.5-flash",
        Pricing {
            input_per_mtok: 0.30,
            output_per_mtok: 2.50,
            cached_input_per_mtok: 0.075,
            input_long_per_mtok: 0.30,
            output_long_per_mtok: 2.50,
            cached_input_long_per_mtok: 0.075,
        },
    ),
    (
        "gemini-2.5-flash-lite",
        Pricing {
            input_per_mtok: 0.10,
            output_per_mtok: 0.40,
            cached_input_per_mtok: 0.025,
            input_long_per_mtok: 0.10,
            output_long_per_mtok: 0.40,
            cached_input_long_per_mtok: 0.025,
        },
    ),
    (
        "gemini-2.0-flash",
        Pricing {
            input_per_mtok: 0.10,
            output_per_mtok: 0.40,
            cached_input_per_mtok: 0.025,
            input_long_per_mtok: 0.10,
            output_long_per_mtok: 0.40,
            cached_input_long_per_mtok: 0.025,
        },
    ),
    (
        "gemini-2.0-flash-lite",
        Pricing {
            input_per_mtok: 0.075,
            output_per_mtok: 0.30,
            cached_input_per_mtok: 0.01875,
            input_long_per_mtok: 0.075,
            output_long_per_mtok: 0.30,
            cached_input_long_per_mtok: 0.01875,
        },
    ),
];

/// Look up the price table entry for a Gemini model id.
///
/// Accepts Vertex resource paths and dated preview/exp snapshots.
pub fn default_pricing(model_id: &str) -> Option<Pricing> {
    let key = normalize_model_id(model_id);
    DEFAULT_PRICING_TABLE
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, p)| *p)
}
