//! # API Module
//!
//! Provides endpoint implementations for API requests.

/// Module that defines API data structures.
pub mod api_types;

/// Module for API root endpoints.
pub mod index_api;

/// Module for the Logs API endpoints.
pub mod logs_api;

// // expose sub modules at this scope
// pub use index_api::echo_endpoint;
// pub use index_api::health_check_endpoint;

// pub use logs_api::get_log_contents_endpoint;
// pub use logs_api::sync_logs_endpoint;
// pub use logs_api::get_log_list_endpoint;
