use super::{
	AttributeError, Generic, ReflectionError, SlangAttribute, Type, UserAttribute, rcall,
	to_cstring,
};
use crate::{GlobalSession, Modifier, ModifierID, succeeded, sys};

#[repr(transparent)]
pub struct Variable(sys::SlangReflectionVariable);

impl Variable {
	pub fn name(&self) -> Option<&str> {
		rcall!(spReflectionVariable_GetName(self) as Option<&str>)
	}

	pub fn ty(&self) -> Option<&Type> {
		rcall!(spReflectionVariable_GetType(self) as Option<&Type>)
	}

	pub fn find_modifier(&self, id: ModifierID) -> Option<&Modifier> {
		rcall!(spReflectionVariable_FindModifier(self, id) as Option<&Modifier>)
	}

	pub fn user_attribute_count(&self) -> u32 {
		rcall!(spReflectionVariable_GetUserAttributeCount(self))
	}

	pub fn user_attribute_by_index(&self, index: u32) -> Option<&UserAttribute> {
		rcall!(spReflectionVariable_GetUserAttribute(self, index) as Option<&UserAttribute>)
	}

	pub fn user_attributes(&self) -> impl ExactSizeIterator<Item = &UserAttribute> {
		(0..self.user_attribute_count()).map(|i| {
			self.user_attribute_by_index(i)
				.expect("index within user_attribute_count should always be valid")
		})
	}

	pub fn find_user_attribute_by_name(
		&self,
		global_session: &GlobalSession,
		name: &str,
	) -> Result<&UserAttribute, ReflectionError> {
		let cname = to_cstring(name)?;
		rcall!(spReflectionVariable_FindUserAttributeByName(
			self,
			global_session as *const _ as *mut _,
			cname.as_ptr()
		) as Option<&UserAttribute>)
		.ok_or_else(|| ReflectionError::NotFound(format!("User attribute '{}'", name)))
	}

	pub fn has_default_value(&self) -> bool {
		rcall!(spReflectionVariable_HasDefaultValue(self))
	}

	pub fn default_value_int(&self) -> Option<i64> {
		let mut value = 0;
		let result = rcall!(spReflectionVariable_GetDefaultValueInt(self, &mut value));
		if succeeded(result) { Some(value) } else { None }
	}

	pub fn generic_container(&self) -> Option<&Generic> {
		rcall!(spReflectionVariable_GetGenericContainer(self) as Option<&Generic>)
	}

	pub fn apply_specializations(&self, generic: &Generic) -> Option<&Variable> {
		rcall!(
			spReflectionVariable_applySpecializations(self, generic as *const _ as *mut _)
				as Option<&Variable>
		)
	}

	/// Extract a typed attribute from the variable at the given index.
	///
	/// This is a convenience method that combines `user_attribute_by_index`
	/// with `SlangAttribute::from_user_attribute` to directly extract typed
	/// attributes based on a trait constraint.
	///
	/// # Examples
	/// ```ignore
	/// #[derive(SlangAttribute)]
	/// #[slang(name = "Range")]
	/// struct RangeAttribute { min: f32, max: f32 }
	///
	/// let range: RangeAttribute = var.extract_attribute(0)?;
	/// ```
	pub fn extract_attribute<T: SlangAttribute>(&self, index: u32) -> Result<T, AttributeError> {
		let attr = self.user_attribute_by_index(index).ok_or_else(|| {
			AttributeError::InvalidValue(format!("No attribute at index {}", index))
		})?;
		T::from_user_attribute(attr)
	}
}
