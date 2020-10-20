use crate::ui::page::Page;
use gtk::{BoxExt, ContainerExt, ListBoxExt, StackExt, WidgetExt};

impl super::Register {
	pub fn new() -> Self {
		let page = Self {
			layout: gtk::Box::new(gtk::Orientation::Horizontal, 0),
			list: gtk::ListBox::new(),
			stack: gtk::Stack::new(),
		};

		// List of items
		page.stack
			.set_transition_type(gtk::StackTransitionType::SlideUpDown);

		// "Welcome" page
		page.stack.add_named(
			super::register_page::Empty::new("Register", Some("document-edit")).widget(),
			"default",
		);

		// Only actual page for now
		page.add_row(
			"Lägenheter",
			"apartments",
			super::register_page::Apartments::new().widget(),
		);

		// Temporary filler
		let temp_pages = [
			("property_owner", "Fastighetsägare"),
			("real_estate", "Fastigheter"),
			("rooms", "Lokaler"),
			("garages", "Garage/p-platser"),
			("tenants", "Hyresgäster"),
			("apartment_types", "Lägenhetstyper"),
		];
		for temp_page in temp_pages.iter() {
			page.add_row(
				temp_page.1,
				temp_page.0,
				super::register_page::Empty::new(temp_page.1, None).widget(),
			);
		}

		page.create_layout();

		page
	}

	fn create_layout(&self) {
		self.layout.pack_start(&self.list, false, false, 0);

		self.layout.pack_start(
			&gtk::Separator::new(gtk::Orientation::Vertical),
			false,
			false,
			0,
		);

		self.layout.pack_start(&self.stack, true, true, 0);

		let stack = self.stack.clone();

		self.list.connect_row_selected(move |_, row| {
			stack.set_visible_child_name(
				&(match row {
					Some(r) => r.get_widget_name().to_string(),
					None => "default".to_string(),
				}),
			)
		});
	}

	fn add_row(&self, title: &str, name: &str, child: &gtk::Widget) {
		// Label to show in list
		let label = gtk::Label::new(Some(title));
		label.set_property_margin(12);
		label.set_halign(gtk::Align::Start);

		// Actual row
		let row = gtk::ListBoxRow::new();
		row.set_widget_name(name);
		row.add(&label);
		self.list.add(&row);

		// Add to stack
		self.stack.add_named(child, name);
	}
}

impl super::Page for super::Register {
	fn widget(&self) -> &gtk::Widget {
		&self.layout.as_ref()
	}
}
