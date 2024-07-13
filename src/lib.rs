//! ```
//! // included str literal
//! let included = include_str!("sample.txt");
//!
//! // raw str literal should match the content of sample.txt
//! let inlined = r#"
//! # this should be hidden by rustdoc
//! this is after the hidden part
//! "#;
//!
//! assert_eq!(dbg!(inlined), dbg!(included));
//! ```
