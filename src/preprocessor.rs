pub struct Preprocessor
{
	source: String,
}

impl Preprocessor
{
	pub fn new(source: String) -> Self
	{
		return Self {
			source,
		};
	}

	pub fn preprocess(self) -> String
	{
		return self.source;
	}
}