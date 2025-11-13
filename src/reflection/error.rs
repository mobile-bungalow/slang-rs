use thiserror::Error;

/// Errors that can occur when working with Slang reflection API
#[derive(Debug, Error, Clone, PartialEq)]
pub enum ReflectionError {
	/// Index was out of bounds for the collection
	#[error("Index {index} out of bounds (size: {size})")]
	IndexOutOfBounds { index: u32, size: u32 },

	/// Expected a different type than what was found
	#[error("Type mismatch: expected {expected}, got {actual}")]
	TypeMismatch { expected: String, actual: String },

	/// Slang C API returned an unexpected NULL pointer
	#[error("Unexpected NULL pointer returned from Slang API")]
	UnexpectedNull,

	/// Failed to convert between types
	#[error("Failed to deserialize value: {0}")]
	DeserializationError(String),

	/// Attribute-specific errors
	#[error("Attribute error: {0}")]
	AttributeError(String),

	/// String contains null byte (invalid for CString)
	#[error(
		"String contains null byte at position {position} (CString requires null-terminated strings without interior nulls)"
	)]
	InvalidString { position: usize },

	/// Item not found by name
	#[error("Not found: {0}")]
	NotFound(String),
}

/// Errors specific to attribute value deserialization
#[derive(Debug, Error, Clone, PartialEq)]
pub enum AttributeError {
	#[error("Type mismatch: expected {expected}, got {actual}")]
	TypeMismatch { expected: String, actual: String },

	#[error("Missing required field: {0}")]
	MissingField(String),

	#[error("Invalid value: {0}")]
	InvalidValue(String),

	#[error("Index {index} out of bounds for array of size {size}")]
	IndexOutOfBounds { index: usize, size: usize },

	#[error("Unexpected NULL from Slang reflection API")]
	UnexpectedNull,
}

impl From<AttributeError> for ReflectionError {
	fn from(err: AttributeError) -> Self {
		ReflectionError::AttributeError(err.to_string())
	}
}
