use super::{AttributeError, UserAttribute};

/// Primary trait for extracting typed data from Slang user attributes
///
/// This is the main API for deserializing attribute data. Use `#[derive(SlangAttribute)]`
/// to automatically implement this trait for your attribute structs.
///
/// # Supported Types
///
/// Due to Slang reflection API limitations, only **scalar types** can be extracted:
/// - `String` - string literals
/// - `f32` - float literals
/// - `i32` - integer literals
///
/// **Not supported:** `Vec<T>`, arrays, other numeric types, or nested structs.
/// Slang's C API only provides `GetArgumentValueInt`, `GetArgumentValueFloat`, and
/// `GetArgumentValueString` which extract literal scalar values.
///
/// # Name Validation
///
/// You can optionally validate the attribute name using `#[slang(name = "...")]`:
///
/// ```ignore
/// #[derive(SlangAttribute)]
/// #[slang(name = "Range")]
/// struct RangeAttribute {
///     min: f32,
///     max: f32,
/// }
/// ```
///
/// This will check that `attr.name() == "Range"` during deserialization.
/// If no `#[slang(name = "...")]` attribute is provided, it defaults to the struct name.
///
/// # Helper Function
///
/// Use the `Variable::extract_attribute<T>()` helper for convenient extraction:
///
/// ```ignore
/// // Instead of:
/// let attr = var.user_attribute_by_index(0).unwrap();
/// let range = RangeAttribute::from_user_attribute(attr)?;
///
/// // You can write:
/// let range: RangeAttribute = var.extract_attribute(0)?;
/// ```
///
/// # Examples
///
/// Basic struct with scalar fields:
///
/// ```ignore
/// use shader_slang::reflection::SlangAttribute;
///
/// #[derive(SlangAttribute)]
/// struct RangeAttribute {
///     min: f32,
///     max: f32,
/// }
///
/// // In your shader: [Range(0.0, 10.0)]
/// let range = RangeAttribute::from_user_attribute(attr)?;
/// ```
///
/// String support:
///
/// ```ignore
/// #[derive(SlangAttribute)]
/// struct LabelAttribute {
///     text: String,
/// }
///
/// // In your shader: [Label("my_label")]
/// let label = LabelAttribute::from_user_attribute(attr)?;
/// ```
///
/// Enum discrimination by attribute name:
///
/// ```ignore
/// #[derive(SlangAttribute)]
/// enum ShaderAttribute {
///     Range(RangeAttribute),
///     Label(LabelAttribute),
/// }
///
/// // Automatically selects the correct variant based on attr.name()
/// let attr = ShaderAttribute::from_user_attribute(user_attr)?;
/// match attr {
///     ShaderAttribute::Range(r) => println!("Range: {} to {}", r.min, r.max),
///     ShaderAttribute::Label(l) => println!("Label: {}", l.text),
/// }
/// ```
pub trait SlangAttribute: Sized {
	fn from_user_attribute(attr: &UserAttribute) -> Result<Self, AttributeError>;
}
