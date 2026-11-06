//! Serializable client/server contracts; no UI, database, or transport dependency.
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageRequest { pub page: u32, pub per_page: u32 }
impl Default for PageRequest { fn default() -> Self { Self { page: 1, per_page: 20 } } }
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Page<T> { pub items: Vec<T>, pub page: u32, pub per_page: u32, pub total_items: u64 }
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiError { pub code: String, pub message: String, pub request_id: Option<String> }
#[cfg(test)] mod tests { use super::*; #[test] fn defaults_are_safe() { assert_eq!(PageRequest::default(), PageRequest { page: 1, per_page: 20 }); } #[test] fn error_serializes() { let e = ApiError { code: "invalid_input".into(), message: "Invalid request".into(), request_id: None }; assert!(serde_json::to_string(&e).unwrap().contains("invalid_input")); } }
