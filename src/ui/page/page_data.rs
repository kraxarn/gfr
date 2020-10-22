pub(crate) struct PageData {
	pub name: String,
	pub title: String,
	pub page: Box<dyn super::register_page::RegisterPage>,
	pub show_arrow: bool,
}

impl PageData {
	pub fn new(
		name: &str,
		title: &str,
		show_arrow: bool,
		page: Box<dyn super::register_page::RegisterPage>,
	) -> Self {
		Self {
			name: name.to_string(),
			title: title.to_string(),
			page,
			show_arrow,
		}
	}
}
