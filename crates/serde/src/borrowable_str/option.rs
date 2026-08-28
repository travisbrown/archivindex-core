//! An optional string, borrowed from the input when possible.
//!
//! # Examples
//!
//! ```
//! use std::borrow::Cow;
//!
//! #[derive(serde::Deserialize)]
//! struct Record<'a> {
//!     #[serde(borrow, with = "archivindex_serde::borrowable_str::option")]
//!     name: Option<Cow<'a, str>>,
//! }
//!
//! let record = serde_json::from_str::<Record<'_>>(r#"{"name":"plain"}"#)?;
//! assert!(matches!(record.name, Some(Cow::Borrowed("plain"))));
//! # Ok::<(), serde_json::Error>(())
//! ```

use std::borrow::Cow;

use serde::de::{Deserialize, Deserializer};

/// Serialize an optional string.
///
/// # Arguments
///
/// * `value` - The string to write, or `None` to write a null
/// * `serializer` - The serializer to write to
///
/// # Returns
///
/// Whatever the serializer returns for the value
///
/// # Errors
///
/// Returns the serializer's own error if writing fails.
pub fn serialize<S: serde::ser::Serializer>(
    value: &Option<Cow<'_, str>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serde::ser::Serialize::serialize(value, serializer)
}

/// Deserialize an optional string, borrowing from the input when possible.
///
/// # Arguments
///
/// * `deserializer` - The deserializer to read an optional string from
///
/// # Returns
///
/// The string, borrowed from the input where the format allowed it, or `None` for a null or
/// missing value.
///
/// # Errors
///
/// Returns the deserializer's own error if the value is present but is not a string.
pub fn deserialize<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Cow<'de, str>>, D::Error> {
    // The stock `Option` implementation supplies the null-or-present layer, leaving
    // `BorrowableStr` to do the borrowing that `Cow`'s own implementation will not.
    Ok(
        Option::<crate::BorrowableStr<'de>>::deserialize(deserializer)?
            .map(|crate::BorrowableStr(value)| value),
    )
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    #[derive(serde::Deserialize, serde::Serialize)]
    struct Record<'a> {
        #[serde(borrow, with = "crate::borrowable_str::option")]
        name: Option<Cow<'a, str>>,
    }

    #[test]
    fn a_present_value_borrows_through_the_option() {
        // Arrange.
        let input = r#"{"name":"plain"}"#;

        // Act.
        let record = serde_json::from_str::<Record<'_>>(input).unwrap();

        // Assert.
        assert!(matches!(record.name, Some(Cow::Borrowed("plain"))));
    }

    #[test]
    fn a_null_value_is_none() {
        // Arrange.
        let input = r#"{"name":null}"#;

        // Act.
        let record = serde_json::from_str::<Record<'_>>(input).unwrap();

        // Assert.
        assert_eq!(record.name, None);
    }

    #[test]
    fn a_record_round_trips() {
        // Arrange: the serializing half must match what the derive would emit on its own.
        let input = r#"{"name":"plain"}"#;

        // Act.
        let record = serde_json::from_str::<Record<'_>>(input).unwrap();
        let output = serde_json::to_string(&record).unwrap();

        // Assert.
        assert_eq!(output, input);
    }
}
