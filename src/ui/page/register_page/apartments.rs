use crate::ui::page::Page;
use gtk::{BoxExt, GridExt, StackExt, StackSwitcherExt, WidgetExt};

const MARGIN: u32 = 12;

struct Notes {
	layout: gtk::Box,
	stack: gtk::Stack,
	switcher: gtk::StackSwitcher,
}

impl super::Apartments {
	pub fn new() -> Self {
		let page = Self {
			layout: gtk::Box::new(gtk::Orientation::Vertical, 16),
			base_options: gtk::Grid::new(),
		};

		page.init_base_options();

		page.layout.pack_start(&page.base_options, false, false, 0);
		page.layout.set_property_margin(32);
		page.layout.set_halign(gtk::Align::Center);

		page
	}

	fn init_base_options(&self) {
		self.base_options.set_column_spacing(MARGIN);
		self.base_options.set_row_spacing(MARGIN);

		let labels = [
			vec![
				"Kontraktsnummer",
				"Objektnummer",
				"Hyresgäst",
				"Person/Org. nr.",
				"Fastighetsbeteckning",
				"Adress",
				"Postnummer",
			],
			vec!["Telefon", "E-post"],
		];

		let mut r = 0;
		let mut c = 0;

		for row in labels.iter() {
			for label in row.iter() {
				self.add_row(label, c, r);
				r += 1;
			}
			r = 0;
			c += 2;
		}

		self.base_options.attach(self.notes().widget(), 2, 2, 2, 5);
	}

	fn add_row(&self, label: &str, column: i32, row: i32) {
		let label = gtk::Label::new(Some(label));
		label.set_halign(gtk::Align::Start);
		self.base_options.attach(&label, column, row, 1, 1);

		let entry = gtk::Entry::new();
		self.base_options.attach(&entry, column + 1, row, 1, 1);
	}

	fn notes(&self) -> Notes {
		let notes = Notes {
			layout: gtk::Box::new(gtk::Orientation::Vertical, 4),
			stack: gtk::Stack::new(),
			switcher: gtk::StackSwitcher::new(),
		};

		notes.switcher.set_stack(Some(&notes.stack));

		notes.layout.pack_start(&notes.switcher, false, false, 0);
		notes.layout.pack_start(&notes.stack, true, true, 0);

		notes
			.stack
			.add_titled(&gtk::TextView::new(), "Egen", "Egen");
		notes
			.stack
			.add_titled(&gtk::TextView::new(), "Faktura", "Faktura");

		notes
	}
}

impl crate::ui::page::Page for super::Apartments {
	fn widget(&self) -> &gtk::Widget {
		self.layout.as_ref()
	}
}

impl crate::ui::page::Page for Notes {
	fn widget(&self) -> &gtk::Widget {
		self.layout.as_ref()
	}
}
