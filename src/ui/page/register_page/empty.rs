use gtk::{ContainerExt, ImageExt, LabelExt, WidgetExt};

impl super::Empty {
	pub fn new(label: &str, icon_name: Option<&str>) -> Self {
		let page = Self {
			layout: gtk::Box::new(gtk::Orientation::Vertical, 16),
		};

		let icon = gtk::Image::from_icon_name(icon_name, gtk::IconSize::Dialog);
		icon.set_pixel_size(128);
		page.layout.add(&icon);

		let label = gtk::Label::new(Some(&format!("<span size='xx-large'>{}</span>", label)));
		label.set_use_markup(true);

		page.layout.add(&label);
		page.layout.set_valign(gtk::Align::Center);

		page
	}
}

impl crate::ui::page::Page for super::Empty {
	fn widget(&self) -> &gtk::Widget {
		self.layout.as_ref()
	}
}
