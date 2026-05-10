//! # gemini-cost
//!
//! Calculate Google Gemini API call cost from a usage block.
//!
//! Gemini's `usageMetadata` block reports `promptTokenCount`,
//! `candidatesTokenCount`, and `cachedContentTokenCount`. Some models
//! charge a different rate when `promptTokenCount > 200_000`. This crate
//! gives you a small `Pricing` table for popular Gemini models plus a
//! `Usage` struct that knows how to compute cost from those fields.
//!
//! Pricing is best-effort and dated; verify against
//! <https://ai.google.dev/gemini-api/docs/pricing> before using these
//! numbers for billing.
//!
//! ## Quick example
//!
//! ```
//! use gemini_cost::{Usage, default_pricing};
//!
//! let pricing = default_pricing("gemini-2.5-pro").unwrap();
//! let usage = Usage {
//!     input_tokens: 1_000,
//!     output_tokens: 500,
//!     cached_input_tokens: 0,
//! };
//! let cost = pricing.cost_for(&usage);
//! assert!(cost > 0.0);
//! ```
//!
//! ## Long-prompt tiers
//!
//! Gemini 2.5 Pro switches to a higher rate when `input_tokens` exceeds
//! 200,000. The pricing table carries both tiers; `cost_for` picks the
//! right one automatically.
//!
//! ```
//! use gemini_cost::{Pricing, Usage, default_pricing};
//! let p = default_pricing("gemini-2.5-pro").unwrap();
//! let short = p.cost_for(&Usage { input_tokens: 100_000, output_tokens: 0, cached_input_tokens: 0 });
//! let long  = p.cost_for(&Usage { input_tokens: 300_000, output_tokens: 0, cached_input_tokens: 0 });
//! assert!(long > short * 3.0); // long-tier is more than the 3x scale alone
//! ```
//!
//! ## BYO pricing
//!
//! ```
//! use gemini_cost::{Pricing, Usage};
//! let custom = Pricing::flat(0.5, 2.0, 0.05);
//! let _ = custom.cost_for(&Usage::default());
//! ```

#![deny(missing_docs)]

mod normalize;
mod pricing;
mod usage;

pub use normalize::normalize_model_id;
pub use pricing::{default_pricing, Pricing, LONG_PROMPT_THRESHOLD, DEFAULT_PRICING_TABLE};
pub use usage::Usage;
