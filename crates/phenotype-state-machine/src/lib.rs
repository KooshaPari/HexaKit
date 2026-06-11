//! Generic finite state machine with transition guards and callbacks.
//!
//! ## Quick start
//!
//! Build a traffic-light style state machine, send a single event,
//! and inspect the new state:
//!
//! ```
//! use phenotype_state_machine::StateMachineBuilder;
//!
//! let sm = StateMachineBuilder::new("red")
//!     .transition("red", "next", "green")
//!     .transition("green", "next", "yellow")
//!     .transition("yellow", "next", "red")
//!     .build()
//!     .unwrap();
//!
//! assert_eq!(sm.current(), "red");
//! let landed = sm.send("next").unwrap();
//! assert_eq!(landed, "green");
//! assert_eq!(sm.current(), "green");
//!
//! // `can_send` reports whether an event is wired from the current state,
//! // and `available_events` returns every wired event.
//! assert!(sm.can_send("next"));
//! assert_eq!(sm.available_events(), vec!["next".to_string()]);
//! ```

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};
use thiserror::Error;

/// Callback type for state enter/exit hooks.
type StateCallback = Arc<dyn Fn(&str) + Send + Sync>;

/// Guard function type for conditional transitions.
type TransitionGuard = Box<dyn Fn(&str, &str) -> bool + Send + Sync>;

/// Errors that can occur during state machine operations.
///
/// Each variant has a stable `Display` rendering suitable for logs
/// and error messages.
///
/// # Examples
///
/// ```
/// use phenotype_state_machine::StateMachineError;
///
/// // No transition is registered for the (state, event) pair.
/// let invalid = StateMachineError::InvalidTransition {
///     from: "red".to_string(),
///     event: "fly".to_string(),
/// };
/// assert_eq!(
///     invalid.to_string(),
///     "invalid transition: no transition from 'red' on event 'fly'",
/// );
///
/// // The transition exists but its guard returned `false`.
/// let rejected = StateMachineError::GuardRejected {
///     from: "locked".to_string(),
///     event: "unlock".to_string(),
/// };
/// assert_eq!(
///     rejected.to_string(),
///     "transition from 'locked' on 'unlock' rejected by guard",
/// );
///
/// // The state is not present in the transition table.
/// assert_eq!(
///     StateMachineError::UnknownState("ghost".to_string()).to_string(),
///     "unknown state: 'ghost'",
/// );
///
/// // The builder rejected the configuration.
/// assert_eq!(
///     StateMachineError::BuildError("initial state cannot be empty".to_string()).to_string(),
///     "builder error: initial state cannot be empty",
/// );
/// ```
#[derive(Debug, Clone, Error)]
pub enum StateMachineError {
    #[error("invalid transition: no transition from '{from}' on event '{event}'")]
    InvalidTransition { from: String, event: String },

    #[error("transition from '{from}' on '{event}' rejected by guard")]
    GuardRejected { from: String, event: String },

    #[error("unknown state: '{0}'")]
    UnknownState(String),

    #[error("builder error: {0}")]
    BuildError(String),
}

/// Result type for state machine operations.
pub type Result<T> = std::result::Result<T, StateMachineError>;

/// A transition definition with optional guard.
struct Transition {
    to: String,
    guard: Option<TransitionGuard>,
}

/// A generic finite state machine.
///
/// Thread-safe via internal `RwLock`. States and events are string-based
/// for maximum flexibility.
pub struct StateMachine {
    current: RwLock<String>,
    transitions: HashMap<(String, String), Transition>,
    on_enter: HashMap<String, Vec<StateCallback>>,
    on_exit: HashMap<String, Vec<StateCallback>>,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl StateMachine {
    /// Create a new empty state machine.
    ///
    /// The initial state is the empty string; the machine has no
    /// transitions, no guards, and no callbacks. Use
    /// [`StateMachineBuilder`] for the common case where transitions
    /// are known up front.
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_state_machine::{StateMachine, StateMachineError};
    ///
    /// // `new` and `default` produce the same empty machine.
    /// let sm = StateMachine::new();
    /// assert_eq!(sm.current(), "");
    /// assert!(!sm.can_send("any"));
    /// assert!(sm.available_events().is_empty());
    ///
    /// // Sending an event with no transitions is `InvalidTransition`.
    /// let err = sm.send("any").unwrap_err();
    /// assert!(matches!(err, StateMachineError::InvalidTransition { .. }));
    /// ```
    pub fn new() -> Self {
        Self {
            current: RwLock::new(String::new()),
            transitions: HashMap::new(),
            on_enter: HashMap::new(),
            on_exit: HashMap::new(),
        }
    }

    /// Get the current state.
    pub fn current(&self) -> String {
        self.current.read().unwrap().clone()
    }

    /// Send an event to the state machine, potentially triggering a transition.
    ///
    /// On success, returns the new state. The current state is updated
    /// as a side effect. If the (state, event) pair has no registered
    /// transition, returns
    /// [`StateMachineError::InvalidTransition`]; if a transition is
    /// registered but its guard returned `false`, returns
    /// [`StateMachineError::GuardRejected`]. `on_exit` callbacks fire
    /// for the old state and `on_enter` callbacks fire for the new
    /// state before the new state is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_state_machine::{StateMachineBuilder, StateMachineError};
    ///
    /// // The guard on "running -> idle" always returns `false`, so every
    /// // `stop` event hits the guard; `halt` is not in the table at all.
    /// let sm = StateMachineBuilder::new("idle")
    ///     .transition("idle", "go", "running")
    ///     .guarded_transition("running", "stop", "idle", |_from, _event| false)
    ///     .build()
    ///     .unwrap();
    ///
    /// // Happy path: returns the new state and updates `current()`.
    /// assert_eq!(sm.send("go").unwrap(), "running");
    /// assert_eq!(sm.current(), "running");
    ///
    /// // Unknown event from the current state: `InvalidTransition`.
    /// assert!(matches!(
    ///     sm.send("halt").unwrap_err(),
    ///     StateMachineError::InvalidTransition { .. }
    /// ));
    ///
    /// // Registered event whose guard rejects: `GuardRejected`, and the
    /// // state is unchanged.
    /// assert!(matches!(
    ///     sm.send("stop").unwrap_err(),
    ///     StateMachineError::GuardRejected { .. }
    /// ));
    /// assert_eq!(sm.current(), "running");
    /// ```
    pub fn send(&self, event: &str) -> Result<String> {
        let mut current = self.current.write().unwrap();
        let key = (current.clone(), event.to_string());

        let transition =
            self.transitions
                .get(&key)
                .ok_or_else(|| StateMachineError::InvalidTransition {
                    from: current.clone(),
                    event: event.to_string(),
                })?;

        if let Some(guard) = &transition.guard {
            if !guard(&current, event) {
                return Err(StateMachineError::GuardRejected {
                    from: current.clone(),
                    event: event.to_string(),
                });
            }
        }

        // Fire on_exit callbacks for current state.
        if let Some(cbs) = self.on_exit.get(current.as_str()) {
            for cb in cbs {
                cb(&current);
            }
        }

        let new_state = transition.to.clone();

        // Fire on_enter callbacks for new state.
        if let Some(cbs) = self.on_enter.get(&new_state) {
            for cb in cbs {
                cb(&new_state);
            }
        }

        *current = new_state.clone();
        Ok(new_state)
    }

    /// Check if a transition is possible from the current state on the given event.
    pub fn can_send(&self, event: &str) -> bool {
        let current = self.current.read().unwrap();
        self.transitions
            .contains_key(&(current.clone(), event.to_string()))
    }

    /// Get all events valid from the current state.
    pub fn available_events(&self) -> Vec<String> {
        let current = self.current.read().unwrap();
        self.transitions
            .keys()
            .filter(|(from, _)| from == current.as_str())
            .map(|(_, event)| event.clone())
            .collect()
    }
}

impl fmt::Debug for StateMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StateMachine")
            .field("current", &self.current())
            .field("transitions", &self.transitions.len())
            .finish()
    }
}

// Send + Sync are safe because internal state is behind RwLock.
unsafe impl Send for StateMachine {}
unsafe impl Sync for StateMachine {}

/// Builder for constructing a [`StateMachine`].
pub struct StateMachineBuilder {
    initial: String,
    transitions: HashMap<(String, String), Transition>,
    on_enter: HashMap<String, Vec<StateCallback>>,
    on_exit: HashMap<String, Vec<StateCallback>>,
}

impl StateMachineBuilder {
    /// Create a new builder with the given initial state.
    ///
    /// `build` returns a [`StateMachine`] whose `current()` is the
    /// supplied initial state and whose transition table is the union
    /// of every [`transition`](Self::transition) /
    /// [`guarded_transition`](Self::guarded_transition) call. An empty
    /// initial state is rejected with
    /// [`StateMachineError::BuildError`].
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_state_machine::{StateMachineBuilder, StateMachineError};
    ///
    /// // Three transitions in a triangle.
    /// let sm = StateMachineBuilder::new("a")
    ///     .transition("a", "next", "b")
    ///     .transition("b", "next", "c")
    ///     .transition("c", "next", "a")
    ///     .build()
    ///     .unwrap();
    ///
    /// sm.send("next").unwrap();
    /// sm.send("next").unwrap();
    /// assert_eq!(sm.current(), "c");
    /// sm.send("next").unwrap();
    /// assert_eq!(sm.current(), "a");
    ///
    /// // The initial state must be non-empty.
    /// let err = StateMachineBuilder::new("").build().unwrap_err();
    /// assert!(matches!(err, StateMachineError::BuildError(_)));
    /// ```
    pub fn new(initial: &str) -> Self {
        Self {
            initial: initial.to_string(),
            transitions: HashMap::new(),
            on_enter: HashMap::new(),
            on_exit: HashMap::new(),
        }
    }

    /// Add a transition: from `from` state, on `event`, go to `to` state.
    pub fn transition(mut self, from: &str, event: &str, to: &str) -> Self {
        self.transitions.insert(
            (from.to_string(), event.to_string()),
            Transition {
                to: to.to_string(),
                guard: None,
            },
        );
        self
    }

    /// Add a guarded transition.
    ///
    /// The guard receives `(&from_state, &event)` and returns `true`
    /// to allow the transition or `false` to reject it. A rejected
    /// transition leaves the current state unchanged and returns
    /// [`StateMachineError::GuardRejected`]. An event with no
    /// registered transition at all is a different error,
    /// [`StateMachineError::InvalidTransition`].
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_state_machine::{StateMachineBuilder, StateMachineError};
    ///
    /// // Two parallel machines: one with a permissive guard, one with
    /// // a rejecting guard, both on the same (state, event) pair.
    /// let sm_ok = StateMachineBuilder::new("locked")
    ///     .guarded_transition("locked", "open", "unlocked", |_from, _event| true)
    ///     .build()
    ///     .unwrap();
    /// let sm_blocked = StateMachineBuilder::new("locked")
    ///     .guarded_transition("locked", "open", "unlocked", |_from, _event| false)
    ///     .build()
    ///     .unwrap();
    ///
    /// // Permissive guard: the transition fires.
    /// assert_eq!(sm_ok.send("open").unwrap(), "unlocked");
    /// assert_eq!(sm_ok.current(), "unlocked");
    ///
    /// // Rejecting guard: returns `GuardRejected` and the state is unchanged.
    /// let err = sm_blocked.send("open").unwrap_err();
    /// assert!(matches!(err, StateMachineError::GuardRejected { .. }));
    /// assert_eq!(sm_blocked.current(), "locked");
    /// ```
    pub fn guarded_transition(
        mut self,
        from: &str,
        event: &str,
        to: &str,
        guard: impl Fn(&str, &str) -> bool + Send + Sync + 'static,
    ) -> Self {
        self.transitions.insert(
            (from.to_string(), event.to_string()),
            Transition {
                to: to.to_string(),
                guard: Some(Box::new(guard)),
            },
        );
        self
    }

    /// Register a callback for when a state is entered.
    ///
    /// `on_enter` callbacks are invoked once per state entry with the
    /// new state's name as the argument. They fire after the guard
    /// has accepted the transition and after `on_exit` callbacks for
    /// the previous state.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use std::sync::atomic::{AtomicUsize, Ordering};
    /// use phenotype_state_machine::StateMachineBuilder;
    ///
    /// // Count entries into "b" using a shared `AtomicUsize`.
    /// let counter = Arc::new(AtomicUsize::new(0));
    /// let counter_for_cb = counter.clone();
    /// let sm = StateMachineBuilder::new("a")
    ///     .transition("a", "go", "b")
    ///     .on_enter("b", move |_state| {
    ///         counter_for_cb.fetch_add(1, Ordering::SeqCst);
    ///     })
    ///     .build()
    ///     .unwrap();
    ///
    /// // First entry fires the callback.
    /// sm.send("go").unwrap();
    /// assert_eq!(counter.load(Ordering::SeqCst), 1);
    /// assert_eq!(sm.current(), "b");
    /// ```
    pub fn on_enter(
        mut self,
        state: &str,
        callback: impl Fn(&str) + Send + Sync + 'static,
    ) -> Self {
        self.on_enter
            .entry(state.to_string())
            .or_default()
            .push(Arc::new(callback));
        self
    }

    /// Register a callback for when a state is exited.
    ///
    /// `on_exit` callbacks receive the old state's name and are
    /// invoked once per state exit, before `on_enter` callbacks for
    /// the next state. They do not fire for a rejected transition.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use std::sync::atomic::{AtomicUsize, Ordering};
    /// use phenotype_state_machine::StateMachineBuilder;
    ///
    /// // Count exits from "a".
    /// let exits = Arc::new(AtomicUsize::new(0));
    /// let exits_for_cb = exits.clone();
    /// let sm = StateMachineBuilder::new("a")
    ///     .transition("a", "go", "b")
    ///     .on_exit("a", move |_state| {
    ///         exits_for_cb.fetch_add(1, Ordering::SeqCst);
    ///     })
    ///     .guarded_transition("b", "back", "a", |_, _| true)
    ///     .build()
    ///     .unwrap();
    ///
    /// // "a" -> "b" fires the exit hook once.
    /// sm.send("go").unwrap();
    /// assert_eq!(exits.load(Ordering::SeqCst), 1);
    ///
    /// // "b" -> "a" does not fire the "a" exit hook (it's an entry).
    /// sm.send("back").unwrap();
    /// assert_eq!(exits.load(Ordering::SeqCst), 1);
    /// ```
    pub fn on_exit(mut self, state: &str, callback: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_exit
            .entry(state.to_string())
            .or_default()
            .push(Arc::new(callback));
        self
    }

    /// Build the state machine.
    pub fn build(self) -> Result<StateMachine> {
        if self.initial.is_empty() {
            return Err(StateMachineError::BuildError(
                "initial state cannot be empty".into(),
            ));
        }

        Ok(StateMachine {
            current: RwLock::new(self.initial.clone()),
            transitions: self.transitions,
            on_enter: self.on_enter,
            on_exit: self.on_exit,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn traffic_light() -> StateMachine {
        StateMachineBuilder::new("red")
            .transition("red", "next", "green")
            .transition("green", "next", "yellow")
            .transition("yellow", "next", "red")
            .build()
            .unwrap()
    }

    #[test]
    fn basic_transitions() {
        let sm = traffic_light();
        assert_eq!(sm.current(), "red");
        sm.send("next").unwrap();
        assert_eq!(sm.current(), "green");
        sm.send("next").unwrap();
        assert_eq!(sm.current(), "yellow");
        sm.send("next").unwrap();
        assert_eq!(sm.current(), "red");
    }

    #[test]
    fn invalid_transition() {
        let sm = traffic_light();
        let err = sm.send("invalid").unwrap_err();
        assert!(matches!(err, StateMachineError::InvalidTransition { .. }));
    }

    #[test]
    fn can_send() {
        let sm = traffic_light();
        assert!(sm.can_send("next"));
        assert!(!sm.can_send("invalid"));
    }

    #[test]
    fn available_events() {
        let sm = traffic_light();
        assert_eq!(sm.available_events(), vec!["next"]);
    }

    #[test]
    fn guard_allows() {
        let sm = StateMachineBuilder::new("locked")
            .guarded_transition("locked", "unlock", "unlocked", |_, _| true)
            .build()
            .unwrap();
        sm.send("unlock").unwrap();
        assert_eq!(sm.current(), "unlocked");
    }

    #[test]
    fn guard_rejects() {
        let sm = StateMachineBuilder::new("locked")
            .guarded_transition("locked", "unlock", "unlocked", |_, _| false)
            .build()
            .unwrap();
        let err = sm.send("unlock").unwrap_err();
        assert!(matches!(err, StateMachineError::GuardRejected { .. }));
        assert_eq!(sm.current(), "locked");
    }

    #[test]
    fn on_enter_callback() {
        let count = Arc::new(AtomicUsize::new(0));
        let c = count.clone();
        let sm = StateMachineBuilder::new("a")
            .transition("a", "go", "b")
            .on_enter("b", move |_| {
                c.fetch_add(1, Ordering::SeqCst);
            })
            .build()
            .unwrap();
        sm.send("go").unwrap();
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn on_exit_callback() {
        let count = Arc::new(AtomicUsize::new(0));
        let c = count.clone();
        let sm = StateMachineBuilder::new("a")
            .transition("a", "go", "b")
            .on_exit("a", move |_| {
                c.fetch_add(1, Ordering::SeqCst);
            })
            .build()
            .unwrap();
        sm.send("go").unwrap();
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn empty_initial_state_errors() {
        let err = StateMachineBuilder::new("").build().unwrap_err();
        assert!(matches!(err, StateMachineError::BuildError(_)));
    }

    #[test]
    fn thread_safety() {
        let sm = Arc::new(traffic_light());
        let handles: Vec<_> = (0..4)
            .map(|_| {
                let sm = sm.clone();
                std::thread::spawn(move || {
                    for _ in 0..100 {
                        let _ = sm.send("next");
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        let state = sm.current();
        assert!(["red", "green", "yellow"].contains(&state.as_str()));
    }
}
