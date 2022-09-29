use std::fmt;
use std::fmt::Debug;
use std::path::PathBuf;
use std::process::exit;

#[derive(Clone, Debug, Default)]
struct OptionsBuilder {
	input: Option<String>,
	output: Option<String>,
}

#[derive(Clone)]
pub struct Options {
	pub input: PathBuf,
	pub output: PathBuf,
}

impl Debug for Options {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} -> {}", self.input.display(), self.output.display())
	}
}

impl Into<Options> for OptionsBuilder {
	fn into(self) -> Options {
		let input = PathBuf::from(self.input.expect("must provide an input file"));
		let output = match (self.output, &input) {
			(Some(output), _) => PathBuf::from(output),
			(None, input) if input == &PathBuf::from("-") => input.clone(),
			(None, input) => {
				let mut output = input.clone();
				output.set_extension("wasm");
				output
			}
		};

		Options { input, output }
	}
}

impl<S> FromIterator<S> for Options
where
	S: AsRef<str>,
{
	fn from_iter<I>(args: I) -> Self
	where
		I: IntoIterator<Item = S>,
	{
		let mut options = OptionsBuilder::default();
		let mut args = args.into_iter();

		while let Some(arg) = args.next() {
			let arg = arg.as_ref();
			if (arg.len() == 2 && arg.starts_with('-')) || arg.len() > 3 && arg.starts_with("--") {
				match arg {
					"-o" | "--output" => {
						options.output = Some(
							args.next()
								.expect(&format!("expected an output file to follow {}", arg))
								.as_ref()
								.to_string(),
						)
					}
					_ => {
						println!("unrecognized option: {}", arg);
						exit(1);
					}
				}
			} else {
				options.input = Some(arg.to_string());
			}
		}

		options.into()
	}
}
