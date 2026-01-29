//! NDJSON Protocol Implementation
//!
//! This module implements the NDJSON (Newline-Delimited JSON) protocol for
//! daemon-client communication.
//!
//! Protocol format:
//! - Each message is a single line of JSON terminated by newline
//! - Request: {"id": "...", "method": "...", "params": {...}}
//! - Response: {"id": "...", "result": {...}, "error": null} or
//!   {"id": "...", "result": null, "error": {"code": "...", "message": "..."}}

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Request message from client to daemon
///
/// Format: `{"id": "unique-id", "method": "method-name", "params": {...}}`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonRequest {
    /// Unique request identifier for correlation
    pub id: String,
    /// Method to invoke (search, status, index)
    pub method: String,
    /// Method-specific parameters
    #[serde(default)]
    pub params: Value,
}

/// Response message from daemon to client
///
/// Format: `{"id": "unique-id", "result": {...}, "error": null}` or
///         `{"id": "unique-id", "result": null, "error": {...}}`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonResponse {
    /// Request identifier for correlation (matches request id)
    pub id: String,
    /// Success response data (null on error)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    /// Error details (null on success)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorInfo>,
}

/// Error information in response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    /// Error code (E001, E002, E003)
    pub code: String,
    /// Human-readable error message
    pub message: String,
}

impl DaemonResponse {
    /// Create a success response
    pub fn success(id: String, result: Value) -> Self {
        Self {
            id,
            result: Some(result),
            error: None,
        }
    }

    /// Create an error response
    pub fn error(id: String, code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            id,
            result: None,
            error: Some(ErrorInfo {
                code: code.into(),
                message: message.into(),
            }),
        }
    }
}

/// Trait for handling protocol messages
pub trait ProtocolHandler: Send + Sync {
    /// Handle a parsed request and return a response
    fn handle_request(&self, request: DaemonRequest) -> DaemonResponse;
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_request_serialization() {
        let request = DaemonRequest {
            id: "test-id".to_string(),
            method: "status".to_string(),
            params: json!({}),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("test-id"));
        assert!(json.contains("status"));
    }

    #[test]
    fn test_response_success() {
        let response = DaemonResponse::success("test-id".to_string(), json!({"status": "ready"}));
        assert_eq!(response.id, "test-id");
        assert!(response.result.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_response_error() {
        let response = DaemonResponse::error("test-id".to_string(), "E001", "Index not ready");
        assert_eq!(response.id, "test-id");
        assert!(response.result.is_none());
        assert!(response.error.is_some());
        let error = response.error.unwrap();
        assert_eq!(error.code, "E001");
    }
}
