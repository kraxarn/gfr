use crate::ui::page::Page;
use gtk::{BoxExt, ContainerExt, WidgetExt};

impl super::Register {
	pub fn new() -> Self {
		let page = Self {
			layout: gtk::Box::new(gtk::Orientation::Horizontal, 0),
			list: gtk::ListBox::new(),
		};

		// List of items
		page.add_row("Lägenheter");

		page.layout.pack_start(&page.list, false, false, 0);

		page.layout.pack_start(
			&gtk::Separator::new(gtk::Orientation::Vertical),
			false,
			false,
			0,
		);

		page.layout.pack_start(
			super::register_page::Apartments::new().widget(),
			true,
			true,
			0,
		);

		page
	}

	fn add_row(&self, title: &str) {
		let label = gtk::Label::new(Some(title));
		label.set_property_margin(12);

		let row = gtk::ListBoxRow::new();
		row.add(&label);

		self.list.add(&row);
	}
}

impl super::Page for super::Register {
	fn widget(&self) -> &gtk::Widget {
		&self.layout.as_ref()
	}
}
