//! Clock is host infrastructure, never an input to an action value.
#[cfg(not(target_arch = "wasm32"))]
pub use std::time::Instant;

#[cfg(target_arch = "wasm32")]
mod browser {
    use std::{cell::Cell, ops::Add, time::Duration};
    thread_local! {
        static CLOCK: Cell<Option<fn() -> Duration>> = const { Cell::new(None) };
    }
    /// The adapter must install a monotonic clock before invoking a solver.
    /// Missing clocks fail explicitly rather than silently disabling budgets.
    pub fn install(clock: fn() -> Duration) {
        CLOCK.with(|c| c.set(Some(clock)));
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Instant(Duration);
    impl Instant {
        pub fn now() -> Self {
            Self(CLOCK.with(|c| c.get().expect("host clock not installed")()))
        }
        pub fn elapsed(self) -> Duration {
            Self::now().0.saturating_sub(self.0)
        }
    }
    impl Add<Duration> for Instant {
        type Output = Self;
        fn add(self, rhs: Duration) -> Self {
            Self(self.0.saturating_add(rhs))
        }
    }
}
#[cfg(target_arch = "wasm32")]
pub use browser::{install, Instant};
