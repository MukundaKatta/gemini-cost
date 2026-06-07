//! Strip Vertex AI publisher prefixes and dated suffixes so the same
//! pricing table works for the AI Studio and Vertex AI endpoints.
//!
//! Gemini model ids come in three shapes:
//!
//! * `gemini-2.5-pro` — AI Studio alias
//! * `gemini-2.5-pro-preview-05-06` — dated AI Studio preview snapshot
//! * `projects/p/locations/l/publishers/google/models/gemini-2.5-pro` —
//!   fully-qualified Vertex resource path
//!
//! We normalize all three to the AI Studio alias.

/// Strip a Vertex resource-path prefix and any trailing
/// `-(preview|exp)-MM-DD` snapshot suffix.
pub fn normalize_model_id(id: &str) -> &str {
    let mut s = id;

    // Vertex resource path -> tail after final `/`
    if s.contains("/publishers/google/models/") {
        if let Some(slash) = s.rfind('/') {
            s = &s[slash + 1..];
        }
    }

    // Strip a trailing -preview-MM-DD or -exp-MM-DD snapshot suffix.
    // The shape is: …-<token>-DD where token in {preview, exp} and the
    // segment before is MM (2 digits).
    if let Some((head, dd)) = s.rsplit_once('-') {
        if is_n_digits(dd, 2) {
            if let Some((head2, mm)) = head.rsplit_once('-') {
                if is_n_digits(mm, 2) {
                    if let Some((head3, tag)) = head2.rsplit_once('-') {
                        if tag == "preview" || tag == "exp" {
                            s = head3;
                        }
                    }
                }
            }
        }
    }

    s
}

fn is_n_digits(s: &str, n: usize) -> bool {
    s.len() == n && s.bytes().all(|b| b.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_vertex_resource_path() {
        assert_eq!(
            normalize_model_id(
                "projects/p/locations/us-central1/publishers/google/models/gemini-2.5-pro"
            ),
            "gemini-2.5-pro"
        );
    }

    #[test]
    fn strips_dated_preview() {
        assert_eq!(
            normalize_model_id("gemini-2.5-pro-preview-05-06"),
            "gemini-2.5-pro"
        );
        assert_eq!(
            normalize_model_id("gemini-2.0-flash-exp-12-01"),
            "gemini-2.0-flash"
        );
    }

    #[test]
    fn keeps_plain_alias() {
        assert_eq!(normalize_model_id("gemini-2.5-pro"), "gemini-2.5-pro");
        assert_eq!(normalize_model_id("gemini-2.5-flash"), "gemini-2.5-flash");
    }
}
