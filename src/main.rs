use ::std::env;
use ::std::fs;
use ::std::io;
use ::std::path::PathBuf;
use ::std::process::exit;

#[derive(Clone, Debug, Default)]
struct OptionsBuilder {
    input: Option<String>,
    output: Option<String>,
}

#[derive(Clone, Debug)]
struct Options {
    input: PathBuf,
    output: PathBuf,
}

impl Into<Options> for OptionsBuilder {
    fn into(self) -> Options {
        let input = PathBuf::from(self.input.expect("Must provide an input file"));

        Options {
            output: match (self.output, &input) {
                (Some(output), _) => PathBuf::from(output),
                (None, input) if input == &PathBuf::from("-") => input.clone(),
                (None, input) => {
                    let mut output = input.clone();
                    output.set_extension("wasm");
                    output
                }
            },
            input,
        }
    }
}

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);
    let mut ob = OptionsBuilder::default();

    while let Some(arg) = args.next() {
        if (arg.len() == 2 && arg.starts_with('-')) || args.len() > 3 && arg.starts_with("--") {
            match arg.as_ref() {
                "-o" | "--output" => {
                    ob.output = Some(args.next().expect("Expected an output file to follow -o"))
                }
                _ => {
                    println!("Unrecognized option: {}", arg);
                    exit(1);
                }
            }
        } else {
            ob.input = Some(arg);
        }
    }

    let options: Options = ob.into();
    println!("Hello, computer! {:?}", options);

    let bytes = ::wat::parse_file(options.input).unwrap();
    fs::write(options.output, bytes)?;

    Ok(())
}
