use gtk::prelude::Cast;
use gtk::{BoxExt, GridExt, WidgetExt};

const MARGIN: u32 = 12;

enum InputType {
	Entry,
	ComboBox,
	CheckButton,
	SpinButton,
}

impl super::Apartments {
	pub fn new() -> Self {
		let page = Self {
			layout: gtk::Box::new(gtk::Orientation::Vertical, 16),
			base_options: gtk::Grid::new(),
			list: gtk::ListBox::new(),
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
				("Objektnummer", InputType::Entry),
				("Lägenhetsnummer", InputType::Entry),
				("Fastighetsbeteckning", InputType::ComboBox),
				("Adress", InputType::ComboBox),
				("Postadress", InputType::ComboBox),
				("Lägenhetstyp", InputType::ComboBox),
				("Storlek kvm", InputType::SpinButton),
				("Våning", InputType::SpinButton),
			],
			vec![
				("Hyra/månad", InputType::Entry),
				("Årshyra", InputType::Entry),
				("Hyra/kvm", InputType::Entry),
				("Övrigt", InputType::Entry),
				("Vind", InputType::Entry),
				("Källare", InputType::Entry),
				("Förråd", InputType::Entry),
				("Elanläggning nr.", InputType::Entry),
				("Postbox nr.", InputType::Entry),
			],
			vec![
				("Uthyrd", InputType::CheckButton),
				("Hiss", InputType::CheckButton),
				("Värme ingår", InputType::CheckButton),
				("Hushållsel ingår", InputType::CheckButton),
				("Trappstädning ingår", InputType::CheckButton),
				("Varmvatten ingår hela året", InputType::CheckButton),
			],
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
	}

	fn add_row(&self, input: &(&str, InputType), column: i32, row: i32) {
		let label = gtk::Label::new(Some(input.0));
		label.set_halign(gtk::Align::Start);
		self.base_options.attach(&label, column, row, 1, 1);

		let entry: gtk::Widget = match input.1 {
			InputType::Entry => gtk::Entry::new().upcast::<gtk::Widget>(),
			InputType::ComboBox => gtk::ComboBox::new().upcast::<gtk::Widget>(),
			InputType::CheckButton => gtk::CheckButton::new().upcast::<gtk::Widget>(),
			InputType::SpinButton => gtk::SpinButton::new(
				Some(&gtk::Adjustment::new(
					25_f64, 1_f64, 50_f64, 1_f64, 0_f64, 0_f64,
				)),
				1_f64,
				0,
			)
			.upcast::<gtk::Widget>(),
		};

		self.base_options.attach(&entry, column + 1, row, 1, 1);
	}
}

impl super::RegisterPage for super::Apartments {
	fn widget(&self) -> &gtk::Widget {
		self.layout.upcast_ref::<gtk::Widget>()
	}

	fn list(&self) -> Option<&gtk::ListBox> {
		Some(&self.list)
	}
}
