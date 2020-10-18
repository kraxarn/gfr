use gtk::{ContainerExt, ImageExt, LabelExt, WidgetExt};

impl super::Home {
	pub fn new() -> Self {
		let page = Self {
			layout: gtk::Box::new(gtk::Orientation::Vertical, 0),
		};
		let logo = gtk::Image::from_icon_name(crate::APPLICATION_ICON, gtk::IconSize::Dialog);
		logo.set_pixel_size(128);
		page.layout.add(&logo);

		let label = gtk::Label::new(Some("<span size='xx-large'>Välkommen!</span>"));
		label.set_use_markup(true);

		page.layout.add(&label);
		page.layout.set_valign(gtk::Align::Center);
		page
	}
}

impl super::Page for super::Home {
	fn widget(&self) -> &gtk::Widget {
		&self.layout.as_ref()
	}
}
