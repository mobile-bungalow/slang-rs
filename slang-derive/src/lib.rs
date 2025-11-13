use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Lit, Meta, Type, parse_macro_input};

/// Generate code to extract a field value from a UserAttribute at the given index
///
/// Only supports: String, f32, i32
fn generate_field_extraction(field_type: &Type, index: usize) -> proc_macro2::TokenStream {
	let index_u32 = index as u32;

	// Check if it's a known type
<<<<<<< HEAD
	if let Type::Path(type_path) = field_type
		&& let Some(segment) = type_path.path.segments.last()
	{
		let type_name = segment.ident.to_string();

		// Handle String
		if type_name == "String" {
			return quote! {
				attr.argument_value_string(#index_u32)
					.ok_or_else(|| ::shader_slang::reflection::AttributeError::InvalidValue(
						format!("Missing or invalid string value at argument {}", #index_u32)
					))?
					.to_string()
			};
		}

		// Handle f32
		if type_name == "f32" {
			return quote! {
				attr.argument_value_float(#index_u32)
					.ok_or_else(|| ::shader_slang::reflection::AttributeError::InvalidValue(
						format!("Missing or invalid f32 value at argument {}", #index_u32)
					))?
			};
		}

		// Handle i32
		if type_name == "i32" {
			return quote! {
				attr.argument_value_int(#index_u32)
					.ok_or_else(|| ::shader_slang::reflection::AttributeError::InvalidValue(
						format!("Missing or invalid i32 value at argument {}", #index_u32)
					))?
			};
		}

		// Unsupported type
		return quote! {
			compile_error!("Only String, f32, and i32 types are supported in SlangAttribute")
		};
	}

	// Fallback error
	quote! {
		compile_error!("Only String, f32, and i32 types are supported in SlangAttribute")
	}
}

/// Extract the name attribute value from the derive input
fn extract_name_attribute(input: &DeriveInput) -> Option<String> {
	for attr in &input.attrs {
		if attr.path().is_ident("slang")
			&& let Meta::List(meta_list) = &attr.meta
		{
			// Parse the tokens inside #[slang(...)]
			let nested = meta_list.parse_args::<Meta>().ok()?;
			if let Meta::NameValue(name_value) = nested
				&& name_value.path.is_ident("name")
			{
				// The value is an Expr, extract string literal from it
				if let syn::Expr::Lit(expr_lit) = &name_value.value
					&& let Lit::Str(lit_str) = &expr_lit.lit
				{
					return Some(lit_str.value());
				}
			}
		}
	}
	None
}

#[proc_macro_derive(SlangAttribute, attributes(slang))]
pub fn derive_slang_attribute(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = &input.ident;

	// Extract the attribute name if provided, otherwise use struct name
	let expected_name = extract_name_attribute(&input).unwrap_or_else(|| name.to_string());

	let implementation = match &input.data {
		Data::Struct(data) => match &data.fields {
			Fields::Named(fields) => {
				let field_extractions: Vec<_> = fields
					.named
					.iter()
					.enumerate()
					.map(|(i, field)| {
						let field_name = &field.ident;
						let field_type = &field.ty;
						let extraction = generate_field_extraction(field_type, i);

						quote! {
							#field_name: #extraction
						}
					})
					.collect();

				let expected_count = fields.named.len();

				quote! {
					impl ::shader_slang::reflection::SlangAttribute for #name {
						fn from_user_attribute(
							attr: &::shader_slang::reflection::UserAttribute
						) -> Result<Self, ::shader_slang::reflection::AttributeError> {
							// Validate attribute name (using override or struct name)
							let attr_name = attr.name().ok_or_else(|| {
								::shader_slang::reflection::AttributeError::InvalidValue(
									"Attribute has no name".to_string()
								)
							})?;

							if attr_name != #expected_name {
								return Err(::shader_slang::reflection::AttributeError::InvalidValue(
									format!("Expected attribute name '{}', got '{}'", #expected_name, attr_name)
								));
							}

							// Validate argument count
							let arg_count = attr.argument_count();
							let expected = #expected_count as u32;

							if arg_count < expected {
								return Err(::shader_slang::reflection::AttributeError::InvalidValue(
									format!("Expected at least {} arguments, got {}", expected, arg_count)
								));
							}

							Ok(Self {
								#(#field_extractions,)*
							})
						}
					}
				}
			}
			_ => {
				return syn::Error::new_spanned(
					&input.ident,
					"SlangAttribute can only be derived for structs with named fields or enums with tuple variants",
				)
				.to_compile_error()
				.into();
			}
		},
		Data::Enum(data) => {
			// For enums, discriminate based on attribute name
			let variants: Vec<_> = data.variants.iter().collect();

			// Generate match arms for each variant
			let match_arms = variants.iter().map(|variant| {
				let variant_name = &variant.ident;
				let variant_name_str = variant_name.to_string();

				match &variant.fields {
					Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
						// Single unnamed field - extract the inner type and delegate to it
						let inner_type = &fields.unnamed.first().unwrap().ty;
						quote! {
							#variant_name_str => {
								let inner = <#inner_type as ::shader_slang::reflection::SlangAttribute>::from_user_attribute(attr)?;
								Ok(Self::#variant_name(inner))
							}
						}
					}
					_ => {
						syn::Error::new_spanned(
							variant,
							"Enum variants must have exactly one unnamed field (e.g., Variant(InnerType))",
						)
						.to_compile_error()
||||||| 9e26678
=======
	if let Type::Path(type_path) = field_type {
		if let Some(segment) = type_path.path.segments.last() {
			let type_name = segment.ident.to_string();

			// Handle String
			if type_name == "String" {
				return quote! {
					attr.argument_value_string(#index_u32)
						.ok_or_else(|| ::shader_slang::reflection::AttributeError::InvalidValue(
							format!("Missing or invalid string value at argument {}", #index_u32)
						))?
						.to_string()
				};
			}

			// Handle f32
			if type_name == "f32" {
				return quote! {
					attr.argument_value_float(#index_u32)
						.ok_or_else(|| ::shader_slang::reflection::AttributeError::InvalidValue(
							format!("Missing or invalid f32 value at argument {}", #index_u32)
						))?
				};
			}

			// Handle i32
			if type_name == "i32" {
				return quote! {
					attr.argument_value_int(#index_u32)
						.ok_or_else(|| ::shader_slang::reflection::AttributeError::InvalidValue(
							format!("Missing or invalid i32 value at argument {}", #index_u32)
						))?
				};
			}

			// Unsupported type
			return quote! {
				compile_error!("Only String, f32, and i32 types are supported in SlangAttribute")
			};
		}
	}

	// Fallback error
	quote! {
		compile_error!("Only String, f32, and i32 types are supported in SlangAttribute")
	}
}

/// Extract the name attribute value from the derive input
fn extract_name_attribute(input: &DeriveInput) -> Option<String> {
	for attr in &input.attrs {
		if attr.path().is_ident("slang") {
			if let Meta::List(meta_list) = &attr.meta {
				// Parse the tokens inside #[slang(...)]
				let nested = meta_list.parse_args::<Meta>().ok()?;
				if let Meta::NameValue(name_value) = nested {
					if name_value.path.is_ident("name") {
						// The value is an Expr, extract string literal from it
						if let syn::Expr::Lit(expr_lit) = &name_value.value {
							if let Lit::Str(lit_str) = &expr_lit.lit {
								return Some(lit_str.value());
							}
						}
					}
				}
			}
		}
	}
	None
}

#[proc_macro_derive(SlangAttribute, attributes(slang))]
pub fn derive_slang_attribute(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = &input.ident;

	// Extract the attribute name if provided, otherwise use struct name
	let expected_name = extract_name_attribute(&input).unwrap_or_else(|| name.to_string());

	let implementation = match &input.data {
		Data::Struct(data) => match &data.fields {
			Fields::Named(fields) => {
				let field_extractions: Vec<_> = fields
					.named
					.iter()
					.enumerate()
					.map(|(i, field)| {
						let field_name = &field.ident;
						let field_type = &field.ty;
						let extraction = generate_field_extraction(field_type, i);

						quote! {
							#field_name: #extraction
						}
					})
					.collect();

				let expected_count = fields.named.len();

				quote! {
					impl ::shader_slang::reflection::SlangAttribute for #name {
						fn from_user_attribute(
							attr: &::shader_slang::reflection::UserAttribute
						) -> Result<Self, ::shader_slang::reflection::AttributeError> {
							// Validate attribute name (using override or struct name)
							let attr_name = attr.name().ok_or_else(|| {
								::shader_slang::reflection::AttributeError::InvalidValue(
									"Attribute has no name".to_string()
								)
							})?;

							if attr_name != #expected_name {
								return Err(::shader_slang::reflection::AttributeError::InvalidValue(
									format!("Expected attribute name '{}', got '{}'", #expected_name, attr_name)
								));
							}

							// Validate argument count
							let arg_count = attr.argument_count();
							let expected = #expected_count as u32;

							if arg_count < expected {
								return Err(::shader_slang::reflection::AttributeError::InvalidValue(
									format!("Expected at least {} arguments, got {}", expected, arg_count)
								));
							}

							Ok(Self {
								#(#field_extractions,)*
							})
						}
					}
				}
			}
			_ => {
				return syn::Error::new_spanned(
					&input.ident,
					"SlangAttribute can only be derived for structs with named fields or enums with tuple variants",
				)
				.to_compile_error()
				.into();
			}
		},
		Data::Enum(data) => {
			// For enums, discriminate based on attribute name
			let variants: Vec<_> = data.variants.iter().collect();

			// Generate match arms for each variant
			let match_arms = variants.iter().map(|variant| {
				let variant_name = &variant.ident;
				let variant_name_str = variant_name.to_string();

				match &variant.fields {
					Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
						// Single unnamed field - extract the inner type and delegate to it
						let inner_type = &fields.unnamed.first().unwrap().ty;
						quote! {
							#variant_name_str => {
								let inner = <#inner_type as ::shader_slang::reflection::SlangAttribute>::from_user_attribute(attr)?;
								Ok(Self::#variant_name(inner))
							}
						}
					}
					_ => {
						return syn::Error::new_spanned(
							variant,
							"Enum variants must have exactly one unnamed field (e.g., Variant(InnerType))",
						)
						.to_compile_error();
>>>>>>> 50dca330609a567586d01be2d6fa0eee565cbdb3
					}
				}
			});

			quote! {
				impl ::shader_slang::reflection::SlangAttribute for #name {
					fn from_user_attribute(
						attr: &::shader_slang::reflection::UserAttribute
					) -> Result<Self, ::shader_slang::reflection::AttributeError> {
						let attr_name = attr.name().ok_or_else(|| {
							::shader_slang::reflection::AttributeError::InvalidValue(
								"Attribute has no name".to_string()
							)
						})?;

						match attr_name {
							#(#match_arms,)*
							_ => Err(::shader_slang::reflection::AttributeError::InvalidValue(
								format!("Unknown attribute name: {}", attr_name)
							))
						}
					}
				}
			}
		}
		_ => {
			return syn::Error::new_spanned(
				&input.ident,
				"SlangAttribute can only be derived for structs or enums",
			)
			.to_compile_error()
			.into();
		}
	};

	TokenStream::from(implementation)
}
