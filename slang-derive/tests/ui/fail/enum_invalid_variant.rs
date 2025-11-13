use slang_derive::SlangAttribute;

#[derive(SlangAttribute)]
enum ShouldFail {
	InvalidVariant,
	AlsoInvalid { field: i32 },
}

fn main() {}
