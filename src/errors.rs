use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug)]
pub struct DataLoadError {
	pub path: PathBuf
}

impl Display for DataLoadError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let path = self.path.display();
		write!(f, "unable to read configuration at {path}")
	}
}

impl Error for DataLoadError {}
