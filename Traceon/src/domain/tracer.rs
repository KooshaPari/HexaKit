//! Tracer Trait - Port Interface

use async_trait::async_trait;

use super::{SpanContext, SpanKind};

/// Tracer trait - primary port
#[async_trait]
pub trait Tracer: Send + Sync {
    /// Start a new span
    fn span(&self, name: &str) -> Box<dyn SpanHandle>;

    /// Start a child span
    fn span_with_parent(&self, name: &str, parent: &SpanContext) -> Box<dyn SpanHandle>;

    /// Start a span with kind
    fn span_with_kind(&self, name: &str, kind: SpanKind) -> Box<dyn SpanHandle>;

    /// Get the tracer name
    fn name(&self) -> &str;

    /// Get the tracer version
    fn version(&self) -> Option<&str>;
}

/// Span handle for finishing spans
pub trait SpanHandle: Send {
    fn set_attribute(&self, key: String, value: super::AttributeValue);
    fn add_event(&self, name: String);
    fn end(&self);
}

/// Span wrapper for scoped spans
pub struct ScopedSpan<'a> {
    #[allow(dead_code)] // retained to tie span lifetime to its tracer
    tracer: &'a dyn Tracer,
    #[allow(dead_code)] // retained for diagnostics/future use
    name: String,
    span: Box<dyn SpanHandle>,
}

impl<'a> ScopedSpan<'a> {
    pub fn new(tracer: &'a dyn Tracer, name: &str) -> Self {
        let span = tracer.span(name);
        Self {
            tracer,
            name: name.to_string(),
            span,
        }
    }

    pub fn with_attribute(self, key: &str, value: super::AttributeValue) -> Self {
        self.span.set_attribute(key.to_string(), value);
        self
    }

    pub fn with_event(self, name: &str) -> Self {
        self.span.add_event(name.to_string());
        self
    }
}

impl<'a> Drop for ScopedSpan<'a> {
    fn drop(&mut self) {
        self.span.end();
    }
}
