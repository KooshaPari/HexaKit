//! Analytics framework

pub mod client;
pub mod error;
pub mod event;
pub mod traits;

pub use client::AnalyticsClient;
pub use error::{AnalyticsError, Result};
pub use event::AnalyticsEvent;
pub use traits::Trackable;
