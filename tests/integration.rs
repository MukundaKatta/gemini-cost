use gemini_cost::{default_pricing, Pricing, Usage};

#[test]
fn computes_basic_cost_short_prompt() {
    let pricing = default_pricing("gemini-2.5-pro").unwrap();
    // 100k input + 50k output at short-tier rates
    // 100k * $1.25 + 50k * $10 = $0.125 + $0.50 = $0.625
    let usage = Usage {
        input_tokens: 100_000,
        output_tokens: 50_000,
        cached_input_tokens: 0,
    };
    let cost = pricing.cost_for(&usage);
    assert!((cost - 0.625).abs() < 1e-6, "got {cost}");
}

#[test]
fn long_prompt_tier_kicks_in_above_200k() {
    let pricing = default_pricing("gemini-2.5-pro").unwrap();
    // 300k input crosses the 200k threshold; whole prompt is billed at
    // long tier ($2.50/Mtok).
    let usage = Usage {
        input_tokens: 300_000,
        output_tokens: 0,
        cached_input_tokens: 0,
    };
    let cost = pricing.cost_for(&usage);
    // 300_000 * 2.5 / 1_000_000 = 0.75
    assert!((cost - 0.75).abs() < 1e-6, "got {cost}");
}

#[test]
fn short_prompt_tier_is_cheaper_per_token() {
    let pricing = default_pricing("gemini-2.5-pro").unwrap();
    let short = pricing.cost_for(&Usage {
        input_tokens: 200_000,
        output_tokens: 0,
        cached_input_tokens: 0,
    });
    let long = pricing.cost_for(&Usage {
        input_tokens: 200_001,
        output_tokens: 0,
        cached_input_tokens: 0,
    });
    // One extra token, but the whole bill flips to long-tier — so long
    // should be roughly 2x the short-tier number.
    assert!(long > short * 1.9, "short {short} long {long}");
}

#[test]
fn flash_has_no_long_uplift() {
    let pricing = default_pricing("gemini-2.5-flash").unwrap();
    let short = pricing.cost_for(&Usage {
        input_tokens: 100_000,
        output_tokens: 0,
        cached_input_tokens: 0,
    });
    let long = pricing.cost_for(&Usage {
        input_tokens: 300_000,
        output_tokens: 0,
        cached_input_tokens: 0,
    });
    // 3x tokens -> exactly 3x cost (single-tier).
    assert!((long - short * 3.0).abs() < 1e-6);
}

#[test]
fn vertex_resource_path_resolves() {
    let id = "projects/p/locations/us-central1/publishers/google/models/gemini-2.5-pro";
    assert_eq!(default_pricing(id), default_pricing("gemini-2.5-pro"));
}

#[test]
fn dated_preview_resolves() {
    assert_eq!(
        default_pricing("gemini-2.5-pro-preview-05-06"),
        default_pricing("gemini-2.5-pro")
    );
}

#[test]
fn gemini_usage_constructor_subtracts_cached() {
    let u = Usage::from_gemini(1000, 500, 300);
    assert_eq!(u.input_tokens, 700);
    assert_eq!(u.output_tokens, 500);
    assert_eq!(u.cached_input_tokens, 300);
    assert_eq!(u.total_tokens(), 1500);
}

#[test]
fn cached_tokens_count_toward_long_prompt_tier() {
    let pricing = default_pricing("gemini-2.5-pro").unwrap();
    // 150k fresh + 100k cached = 250k total prompt tokens. Gemini's
    // promptTokenCount (250k) crosses the 200k threshold, so the whole
    // request bills at the long tier even though fresh input alone is
    // below the threshold.
    let usage = Usage {
        input_tokens: 150_000,
        output_tokens: 0,
        cached_input_tokens: 100_000,
    };
    // long-tier: 150_000 * 2.5 + 100_000 * 0.625 = 0.375 + 0.0625 = 0.4375
    let cost = pricing.cost_for(&usage);
    assert!((cost - 0.4375).abs() < 1e-6, "got {cost}");
}

#[test]
fn cache_hit_uses_cached_rate() {
    let pricing = default_pricing("gemini-2.5-pro").unwrap();
    // 100k cached tokens: total prompt stays under the 200k threshold, so
    // the short-tier cached rate ($0.3125/Mtok) applies.
    let usage = Usage {
        input_tokens: 0,
        output_tokens: 0,
        cached_input_tokens: 100_000,
    };
    assert!(usage.cache_hit());
    // 100_000 * 0.3125 / 1_000_000 = 0.03125
    let cost = pricing.cost_for(&usage);
    assert!((cost - 0.03125).abs() < 1e-6, "got {cost}");
}

#[test]
fn flat_pricing_helper() {
    let p = Pricing::flat(1.0, 4.0, 0.25);
    assert_eq!(p.input_per_mtok, p.input_long_per_mtok);
    assert_eq!(p.output_per_mtok, p.output_long_per_mtok);
}

#[test]
fn unknown_model_is_none() {
    assert!(default_pricing("gemini-9000").is_none());
}
