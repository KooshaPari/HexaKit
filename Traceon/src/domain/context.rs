//! Trace Context

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// W3C Trace Context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct W3CTraceContext {
    pub traceparent: String,
}

impl W3CTraceContext {
    pub fn new(trace_id: Uuid, span_id: Uuid, sampled: bool) -> Self {
        let flags = if sampled { "01" } else { "00" };
        // W3C traceparent: 00-<32hex trace-id>-<16hex parent-id>-<2hex flags>
        let tid = format!("{:032x}", trace_id.as_u128());
        let sid = format!("{:016x}", (span_id.as_u128() & 0xFFFF_FFFF_FFFF_FFFF) as u64);
        let traceparent = format!("00-{}-{}-{}", tid, sid, flags);
        Self { traceparent }
    }

    pub fn trace_id(&self) -> Option<Uuid> {
        // traceparent = "00-<32hex>-<16hex>-<2hex>"
        let parts: Vec<&str> = self.traceparent.splitn(4, '-').collect();
        if parts.len() >= 2 {
            // Parse 32 hex chars as UUID (no hyphens)
            Uuid::parse_str(parts[1]).ok()
        } else {
            None
        }
    }
}

/// B3 Propagation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B3Context {
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
    pub sampled: Option<bool>,
}

impl B3Context {
    pub fn new(trace_id: Uuid, span_id: Uuid, sampled: bool) -> Self {
        Self {
            trace_id: Some(format!("{:032x}", trace_id.as_u128())),
            span_id: Some(format!("{:016x}", (span_id.as_u128() & 0xFFFF_FFFF_FFFF_FFFF) as u64)),
            sampled: Some(sampled),
        }
    }
}
