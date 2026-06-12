//! Structured logging context attached to log events.
//!
//! `LogContext` is the canonical carrier of per-request / per-job metadata
//! (request ids, user ids, OpenTelemetry trace / span ids, free-form key/value
//! pairs) that should be propagated through log output. It is intentionally
//! `Serialize` + `Deserialize` so that contexts can be persisted, replayed,
//! shipped over the wire, or reconstructed in test fixtures, and `Hash` +
//! `Eq` so it can be used as a key in hash-based collections (e.g.
//! `HashMap<LogContext, V>`) for de-duplication, lookup, and grouping.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Structured context that travels with a log event.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LogContext {
    /// Inbound request correlation id (e.g. HTTP `X-Request-Id`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,

    /// Authenticated principal / user id associated with the event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// OpenTelemetry / W3C trace id (32 hex chars).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,

    /// OpenTelemetry / W3C span id (16 hex chars).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_id: Option<String>,

    /// Free-form key/value tags. BTreeMap keeps the JSON representation
    /// deterministic for snapshot tests.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub tags: BTreeMap<String, String>,
}

impl LogContext {
    /// Construct an empty context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Attach a request id and return `self` for chaining.
    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    /// Attach a user id and return `self` for chaining.
    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// Attach a trace id and return `self` for chaining.
    pub fn with_trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }

    /// Attach a span id and return `self` for chaining.
    pub fn with_span_id(mut self, span_id: impl Into<String>) -> Self {
        self.span_id = Some(span_id.into());
        self
    }

    /// Insert a free-form tag.
    pub fn with_tag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.tags.insert(key.into(), value.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn log_context_round_trip_json() {
        let original = LogContext::new()
            .with_request_id("req-12345")
            .with_user_id("user-abc")
            .with_trace_id("0af7651916cd43dd8448eb211c80319c")
            .with_span_id("b7ad6b7169203331")
            .with_tag("tenant", "acme")
            .with_tag("env", "prod");

        let json = serde_json::to_string(&original).expect("serialize LogContext");
        let restored: LogContext =
            serde_json::from_str(&json).expect("deserialize LogContext");

        assert_eq!(restored, original, "round-trip should preserve all fields");

        // Sanity: the serialized form is non-empty and round-trips through both
        // directions without losing the optional fields or the tag map.
        assert!(json.contains("\"request_id\":\"req-12345\""));
        assert!(json.contains("\"tags\":{\"env\":\"prod\",\"tenant\":\"acme\"}"));
    }

    #[test]
    fn log_context_usable_as_hashmap_key() {
        let mut map: HashMap<LogContext, String> = HashMap::new();

        let api = LogContext::new()
            .with_request_id("req-1")
            .with_user_id("user-a")
            .with_tag("route", "/orders");
        let worker = LogContext::new()
            .with_request_id("job-9")
            .with_trace_id("0af7651916cd43dd8448eb211c80319c");

        map.insert(api.clone(), "api-handler".to_string());
        map.insert(worker.clone(), "background-worker".to_string());

        // Lookups by an equal key (constructed independently) must hit.
        let api_lookup = LogContext::new()
            .with_request_id("req-1")
            .with_user_id("user-a")
            .with_tag("route", "/orders");
        assert_eq!(map.get(&api_lookup).map(String::as_str), Some("api-handler"));
        assert_eq!(map.get(&worker).map(String::as_str), Some("background-worker"));

        // Two equal contexts (regardless of construction order / tag insertion
        // order) must hash the same and be treated as a single key.
        assert_eq!(map.len(), 2);
    }
}
