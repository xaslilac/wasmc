use std::path::Path;
use std::process::Command;

mod macros;
mod setup;
mod util;
use util::needs_dir;

const EXE: &str = "./build/release/wasmc";

#[test]
fn declared_input_inferred_output() {
	setup::before();

	// This file must not exist before testing
	assert!(!Path::new("./tests/testdata/add.wasm").exists());
	// ...and we must make sure to delete it after testing
	delete_after!("./tests/testdata/add.wasm");

	let result = Command::new(EXE)
		.arg("./tests/testdata/add.wat")
		.output()
		.unwrap();
	let stdout = String::from_utf8_lossy(&result.stdout);

	assert_eq!(
		stdout,
		"./tests/testdata/add.wat -> ./tests/testdata/add.wasm\n"
	);
	assert!(Path::new("./tests/testdata/add.wasm").is_file());
}

#[test]
fn declared_input_declared_output() {
	setup::before();

	// This file must not exist before testing
	assert!(!Path::new("./tests/testdata/build/add.wasm").exists());
	// ...and we must make sure to delete it after testing
	delete_after!("./tests/testdata/build/add.wasm");

	needs_dir("./tests/testdata/build/");

	let result = Command::new(EXE)
		.arg("./tests/testdata/add.wat")
		.args(["-o", "./tests/testdata/build/add.wasm"])
		.output()
		.unwrap();
	let stdout = String::from_utf8_lossy(&result.stdout);

	assert_eq!(
		stdout,
		"./tests/testdata/add.wat -> ./tests/testdata/build/add.wasm\n"
	);
	assert!(Path::new("./tests/testdata/build/add.wasm").is_file());
}

#[test]
fn no_file() {
	setup::before();

	let result = Command::new(EXE).output().unwrap();
	let stderr = String::from_utf8_lossy(&result.stderr);

	assert!(stderr.contains("must provide an input file"));
}
