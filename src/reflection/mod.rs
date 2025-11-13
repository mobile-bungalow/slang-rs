mod decl;
mod entry_point;
mod error;
mod function;
mod generic;
pub mod helpers;
mod shader;
mod ty;
mod type_layout;
mod type_parameter;
mod user_attribute;
mod variable;
mod variable_layout;

pub use decl::Decl;
pub use entry_point::EntryPoint;
pub use error::{AttributeError, ReflectionError};
pub use function::Function;
pub use generic::Generic;
pub use helpers::SlangAttribute;
pub use shader::Shader;
pub use ty::Type;
pub use type_layout::TypeLayout;
pub use type_parameter::TypeParameter;
pub use user_attribute::UserAttribute;
pub use variable::Variable;
pub use variable_layout::VariableLayout;

use super::sys;
use std::ffi::CString;

pub fn compute_string_hash(string: &str) -> u32 {
	rcall!(spComputeStringHash(string, string.len()))
}

/// Helper trait to convert strings to CString with proper error handling
trait ToCStringResult {
	fn to_cstring_result(self) -> Result<CString, ReflectionError>;
}

impl ToCStringResult for &str {
	fn to_cstring_result(self) -> Result<CString, ReflectionError> {
		CString::new(self).map_err(|e| ReflectionError::InvalidString {
			position: e.nul_position(),
		})
	}
}

/// Helper function to convert a string to CString with proper error mapping
#[inline]
pub(crate) fn to_cstring(s: &str) -> Result<CString, ReflectionError> {
	s.to_cstring_result()
}

macro_rules! rcall {
	($f:ident($s:ident $(,$arg:expr)*)) => {
		unsafe { sys::$f($s as *const _ as *mut _ $(,$arg)*) }
	};

	($f:ident($s:ident $(,$arg:expr)*) as Option<&str>) => {
		unsafe {
			let ptr = sys::$f($s as *const _ as *mut _ $(,$arg)*);
			(!ptr.is_null()).then(|| std::ffi::CStr::from_ptr(ptr).to_str().ok()).flatten()
		}
	};

	($f:ident($s:ident $(,$arg:expr)*) as Option<&$cast:ty>) => {
		unsafe {
			let ptr = sys::$f($s as *const _ as *mut _ $(,$arg)*);
			(!ptr.is_null()).then(|| &*(ptr as *const $cast))
		}
	};
}

pub(super) use rcall;
