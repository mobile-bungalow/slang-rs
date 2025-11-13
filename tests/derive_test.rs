#![cfg(feature = "derive")]

use shader_slang::Downcast;
use shader_slang::reflection::SlangAttribute;

#[test]
fn derive_slang_attribute() {
	// RangeAttribute: Uses explicit name override "Range"
	#[derive(Debug, PartialEq, shader_slang::SlangAttribute)]
	#[slang(name = "Range")]
	struct RangeAttribute {
		min: f32,
		max: f32,
	}

	#[derive(Debug, PartialEq, shader_slang::SlangAttribute)]
	#[slang(name = "Label")]
	struct LabelAttribute {
		text: String,
	}

	// Enum that discriminates based on attribute name
	#[derive(Debug, PartialEq, shader_slang::SlangAttribute)]
	enum ShaderAttribute {
		Range(RangeAttribute),
		Label(LabelAttribute),
	}

	let shader_source = r#"
[__AttributeUsage(_AttributeTargets.Var)]
struct RangeAttribute {
    float min;
    float max;
};

[__AttributeUsage(_AttributeTargets.Var)]
struct LabelAttribute {
    string text;
};

[Range(0.0, 10.0)]
uniform float intensity;

[Label("test_label")]
uniform float labeled;

[shader("compute")]
[numthreads(1, 1, 1)]
void main(uint3 id : SV_DispatchThreadID) {}
"#;

	let global_session = shader_slang::GlobalSession::new().unwrap();
	let session_options = shader_slang::CompilerOptions::default();
	let target_desc = shader_slang::TargetDesc::default()
		.format(shader_slang::CompileTarget::Spirv)
		.profile(global_session.find_profile("sm_6_0").unwrap());
	let targets = [target_desc];
	let session_desc = shader_slang::SessionDesc::default()
		.targets(&targets)
		.options(&session_options);
	let session = global_session.create_session(&session_desc).unwrap();
	let module = session
		.load_module_from_source_string("derive_test", "derive_test.slang", shader_source)
		.unwrap();
	let entry_point = module.find_entry_point_by_name("main").unwrap();
	let program = session
		.create_composite_component_type(&[
			module.downcast().clone(),
			entry_point.downcast().clone(),
		])
		.unwrap();
	let linked = program.link().unwrap();
	let reflection = linked.layout(0).unwrap();

	// Test Range attribute with explicit name override
	let intensity = reflection.parameter_by_index(0).unwrap();
	let intensity_var = intensity.variable().unwrap();
	let range_attr = intensity_var.user_attribute_by_index(0).unwrap();

	assert_eq!(range_attr.name(), Some("Range"));
	let range = RangeAttribute::from_user_attribute(range_attr).unwrap();
	assert_eq!(
		range,
		RangeAttribute {
			min: 0.0,
			max: 10.0
		}
	);

	// Test Label attribute with String support
	let labeled = reflection.parameter_by_index(1).unwrap();
	let labeled_var = labeled.variable().unwrap();
	let label_attr = labeled_var.user_attribute_by_index(0).unwrap();

	assert_eq!(label_attr.name(), Some("Label"));
	let label = LabelAttribute::from_user_attribute(label_attr).unwrap();
	assert_eq!(
		label,
		LabelAttribute {
			text: "test_label".to_string()
		}
	);

	// Test enum discrimination by attribute name
	let range_enum = ShaderAttribute::from_user_attribute(range_attr).unwrap();
	assert!(matches!(range_enum, ShaderAttribute::Range(_)));

	let label_enum = ShaderAttribute::from_user_attribute(label_attr).unwrap();
	if let ShaderAttribute::Label(inner) = label_enum {
		assert_eq!(inner.text, "test_label");
	} else {
		panic!("Expected Label variant");
	}

	// Test the new helper extraction function
	let range_extracted: RangeAttribute = intensity_var.extract_attribute(0).unwrap();
	assert_eq!(
		range_extracted,
		RangeAttribute {
			min: 0.0,
			max: 10.0
		}
	);

	let label_extracted: LabelAttribute = labeled_var.extract_attribute(0).unwrap();
	assert_eq!(
		label_extracted,
		LabelAttribute {
			text: "test_label".to_string()
		}
	);

	// Test enum extraction via helper
	let range_enum_extracted: ShaderAttribute = intensity_var.extract_attribute(0).unwrap();
	assert!(matches!(range_enum_extracted, ShaderAttribute::Range(_)));
}
