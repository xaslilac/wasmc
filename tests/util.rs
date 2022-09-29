use std::fs::create_dir;
use std::fs::remove_file;
use std::path::Path;

pub struct DeleteAfter<P>(pub P)
where
	P: AsRef<Path>;

impl<P> Drop for DeleteAfter<P>
where
	P: AsRef<Path>,
{
	fn drop(&mut self) {
		_ = remove_file(&self.0).expect("unable to clean up test file");
	}
}

pub fn needs_dir<P>(path: P)
where
	P: AsRef<Path>,
{
	let path = path.as_ref();
	if !path.exists() {
		create_dir(path).expect("could not create directory needed for test");
	}
}
