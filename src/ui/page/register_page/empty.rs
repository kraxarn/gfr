use gtk::BoxExt;

impl super::Empty {
	pub fn new() -> Self {
		let page = Self {
			layout: gtk::Box::new(gtk::Orientation::Vertical, 16),
		};

		page.layout
			.set_center_widget(Some(&gtk::Label::new(Some("Register"))));
		page
	}
}

impl crate::ui::page::Page for super::Empty {
	fn widget(&self) -> &gtk::Widget {
		self.layout.as_ref()
	}
}
