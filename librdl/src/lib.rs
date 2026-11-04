//! Shared library for rudderless.
//!
//! Rudderless is an experiment in simple workload orchestration.
#![warn(missing_docs)]

/// Current crate version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Return the library version.
pub fn version() -> &'static str {
    VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_not_empty() {
        assert!(!version().is_empty());
    }
}
