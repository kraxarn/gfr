use crate::ui::page::Page;
use gtk::{BoxExt, ContainerExt, ImageExt, ListBoxExt, StackExt, WidgetExt};

struct PageData {
	name: String,
	title: String,
	show_arrow: bool,
}

impl PageData {
	fn new(name: &str, title: &str, show_arrow: bool) -> Self {
		Self {
			name: name.to_string(),
			title: title.to_string(),
			show_arrow,
		}
	}
}

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
			&PageData::new("apartments", "Lägenheter", true),
			super::register_page::Apartments::new().widget(),
		);

		// Temporary filler
		let temp_pages = [
			PageData::new("real_estate", "Fastigheter", true),
			PageData::new("rooms", "Lokaler", true),
			PageData::new("garages", "Garage/p-platser", true),
			PageData::new("tenants", "Hyresgäster", true),
			PageData::new("property_owner", "Fastighetsägare", false),
			PageData::new("types", "Typer", false),
		];
		for temp_page in temp_pages.iter() {
			page.add_row(
				temp_page,
				super::register_page::Empty::new(&temp_page.title, None).widget(),
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

	fn add_row(&self, page_data: &PageData, child: &gtk::Widget) {
		// Row layout
		let row = gtk::ListBoxRow::new();
		row.set_widget_name(&page_data.name);
		let layout = gtk::Box::new(gtk::Orientation::Horizontal, 0);
		row.add(&layout);

		// Label
		let label = gtk::Label::new(Some(&page_data.title));
		label.set_property_margin(12);
		label.set_halign(gtk::Align::Start);
		layout.pack_start(&label, true, true, 12);

		// Arrow icon
		let image = gtk::Image::new();
		image.set_from_icon_name(
			if page_data.show_arrow {
				Some("go-next-symbolic")
			} else {
				None
			},
			gtk::IconSize::Button,
		);
		layout.pack_end(&image, false, false, 12);

		// Add to layout
		self.list.add(&row);
		self.stack.add_named(child, &page_data.name);
	}
}

impl super::Page for super::Register {
	fn widget(&self) -> &gtk::Widget {
		&self.layout.as_ref()
	}
}
