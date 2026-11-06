//! Application-independent foundational types.

use thiserror::Error;

/// Errors shared across foundational modules.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum CoreError {
    /// A supplied identifier was empty or malformed.
    #[error("invalid identifier")]
    InvalidIdentifier,
}

/// A small validated identifier suitable for cross-crate use.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Identifier(String);

impl Identifier {
    /// Create an identifier from a non-empty string.
    pub fn new(value: impl Into<String>) -> Result<Self, CoreError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(CoreError::InvalidIdentifier);
        }
        Ok(Self(value))
    }

    /// Borrow the identifier's string representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_blank_identifiers() {
        assert_eq!(Identifier::new("  "), Err(CoreError::InvalidIdentifier));
    }

    #[test]
    fn preserves_valid_identifier() {
        let id = Identifier::new("account_123").expect("valid identifier");
        assert_eq!(id.as_str(), "account_123");
    }
}
