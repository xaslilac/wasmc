#[macro_export]
macro_rules! delete_after {
	( $s:expr ) => {
		let _d = crate::util::DeleteAfter($s);
	};
}
