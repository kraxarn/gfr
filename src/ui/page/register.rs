use crate::ui::page::Page;
use gtk::{BoxExt, StackExt, StackSwitcherExt};

impl super::Register {
	pub fn new() -> Self {
		let page = Self {
			layout: gtk::Box::new(gtk::Orientation::Horizontal, 0),
			switcher: gtk::StackSwitcher::new(),
			stack: gtk::Stack::new(),
		};

		page.switcher.set_stack(Some(&page.stack));

		// List of items
		page.stack.add_titled(
			super::register_page::Apartments::new().widget(),
			"Lägenheter",
			"Lägenheter",
		);

		// Layout
		page.layout.pack_start(&page.switcher, false, false, 0);

		page.layout.pack_start(
			&gtk::Separator::new(gtk::Orientation::Vertical),
			false,
			false,
			0,
		);

		page.layout.pack_start(&page.stack, true, true, 0);

		page
	}
}

impl super::Page for super::Register {
	fn widget(&self) -> &gtk::Widget {
		&self.layout.as_ref()
	}
}
