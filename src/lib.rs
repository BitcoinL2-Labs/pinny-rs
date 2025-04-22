//! # Pinny
//!
//! **Pinny** is a procedural macro crate that enables you to assign tags to your test functions using the `#[tag(...)]` attribute.
//! These tags allow for better test organization and filtering, and they must be declared explicitly in your `Cargo.toml` file under `package.metadata.pinny.allowed`.
//!
//! ## Configuration
//!
//! To use this crate, add it to your `Cargo.toml` as a development dependency, and configure the allowed tags as follows:
//!
//! ```toml
//! [dev-dependencies]
//! pinny = "..."
//!
//! [package.metadata.pinny]
//! allowed = ["allowed_tag1", "allowed_tag2"]
//! ```
//!
//! - The `allowed` field defines a whitelist of tags that can be applied to test functions.
//! - Tags not listed here will result in a **compile-time error**.
//!
//! ## Usage
//!
//! After configuration, you can annotate your test functions with tags like this:
//!
//! ```rust,ignore
//! use pinny::tag;
//!
//! #[tag(allowed_tag1, allowed_tag2)]
//! #[test]
//! fn test_hello() {
//!     assert!(true);
//! }
//!
//! // This will fail to compile because `invalid_tag` is not allowed in Cargo.toml
//! #[tag(invalid_tag)]
//! #[test]
//! fn test_invalid_tag() {
//!     assert!(false);
//! }
//! ```
//!
//! ### Notes
//!
//! - The `#[tag(...)]` macro must appear **before** the `#[test]` attribute.
//! - Only tags declared in `Cargo.toml` under `package.metadata.pinny.allowed` are valid.
//!
//! ## Test Filtering
//!
//! Once the tests are tagged then they can be **filtered and run by tag name**.
//!
//! When a test is tagged, `pinny` rewrites the test's function internally to include the tags as modules. The new format looks like this:
//!
//! ```text
//! <module_path>::<test_name>::t::<tag1>::<tag2>::t
//! ```
//!
//! This transformation enables powerful filtering depending on the test runner used.
//! For example using the built-in `cargo test`:
//!
//! ```bash
//! cargo test :allowed_tag1:
//! ```
//!
//! This command will only run tests tagged with `allowed_tag1`.
//!
//! ### Notes
//!
//! - tags are traslated into module path and are enclosed between `t` delimiters
//! - Depending on the test runner is it possible to apply simple or complex filter to run a specific set of tests.
mod config;
mod tag;
#[cfg(test)]
mod tests;

use proc_macro::TokenStream;

/// A procedural macro for the `tag` attribute.
///
/// The attribute can be used to associate one or more allowed tags
/// (in Cargo.toml) with a test.
///
/// The attribute should be placed before `#[test]` attribute.
///
/// # Example
///
/// Specify the attribute on a per-test basis:
/// ```rust,ignore
/// use pinny::tag;
///
/// #[tag(allowed_tag1, allowed_tag2)]
/// #[test]
/// fn test_hello() {
///   assert!(true);
/// }
/// ```
#[proc_macro_attribute]
pub fn tag(attrs: TokenStream, item: TokenStream) -> TokenStream {
    tag::macro_impl(attrs, item)
}
