pub(crate) struct PageData {
	pub name: String,
	pub title: String,
	pub show_arrow: bool,
	pub page: Box<dyn super::Page>,
}

impl PageData {
	pub fn new(name: &str, title: &str, show_arrow: bool, page: Box<dyn super::Page>) -> Self {
		Self {
			name: name.to_string(),
			title: title.to_string(),
			show_arrow,
			page,
		}
	}
}
