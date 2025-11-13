use slang_derive::SlangAttribute;

#[derive(SlangAttribute)]
#[slang(name = "Range")]
struct RangeAttribute {
	min: f32,
	max: f32,
}

#[derive(SlangAttribute)]
#[slang(name = "Metadata")]
struct MetadataAttribute {
	version: i32,
	scale: f32,
}

#[derive(SlangAttribute)]
enum ShaderAttribute {
	Range(RangeAttribute),
	Metadata(MetadataAttribute),
}

fn main() {}
