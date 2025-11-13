use slang_derive::SlangAttribute;

// Test with explicit name override
#[derive(SlangAttribute)]
#[slang(name = "Range")]
struct RangeAttribute {
	min: f32,
	max: f32,
}

// Test without name attribute (uses struct name "MetadataAttribute")
#[derive(SlangAttribute)]
struct MetadataAttribute {
	version: i32,
	scale: f32,
	flags: i32,
}

fn main() {}
