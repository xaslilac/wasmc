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
