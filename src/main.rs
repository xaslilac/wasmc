use std::env;
use std::fs;
use std::io;

mod options;
use options::Options;

fn main() -> io::Result<()> {
	let options = env::args().skip(1).collect::<Options>();
	println!("{:?}", options);

	let bytes = wat::parse_file(options.input).unwrap();
	fs::write(options.output, bytes)?;

	Ok(())
}
