//! ```
//! // included str literal
//! let included = include_str!("sample.hjson");
//!
//! // raw str literal should match the content of sample.hjson
//! let inlined = r#"{
//!     # specify rate in requests/second
//!     rate: 1000
//!     array:
//!     [
//!         foo
//!         bar
//!     ]
//! }"#;
//!
//! assert_eq!(inlined, included);
//! ```
