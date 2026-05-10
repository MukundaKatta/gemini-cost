//! Token usage block.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Three-field token usage as returned by Gemini's `usageMetadata`.
///
/// Gemini reports `promptTokenCount` as **including** cached content
/// tokens, so the [`Usage::from_gemini`] constructor subtracts
/// `cachedContentTokenCount` from `promptTokenCount` to get the fresh
/// input count this struct expects.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Usage {
    /// Fresh input/prompt tokens (not served from the context cache).
    pub input_tokens: u64,
    /// Output / candidate tokens.
    pub output_tokens: u64,
    /// Input tokens that were served from a context cache (cache hit).
    pub cached_input_tokens: u64,
}

impl Usage {
    /// True when the request hit the context cache.
    pub fn cache_hit(&self) -> bool {
        self.cached_input_tokens > 0
    }

    /// Total tokens billed (input + output + cached_input).
    pub fn total_tokens(&self) -> u64 {
        self.input_tokens + self.output_tokens + self.cached_input_tokens
    }

    /// Build a Usage from a Gemini `usageMetadata` payload.
    ///
    /// `prompt_token_count` includes cached tokens on the wire; this
    /// constructor subtracts them so the struct's fields are disjoint.
    pub fn from_gemini(
        prompt_token_count: u64,
        candidates_token_count: u64,
        cached_content_token_count: u64,
    ) -> Self {
        Self {
            input_tokens: prompt_token_count.saturating_sub(cached_content_token_count),
            output_tokens: candidates_token_count,
            cached_input_tokens: cached_content_token_count,
        }
    }
}
