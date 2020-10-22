use gtk::{BoxExt, ContainerExt, LabelExt, WidgetExt};

pub fn list_item(name: &str, label: Option<&str>, icon: Option<&str>) -> gtk::ListBoxRow {
	// Layout
	let row = gtk::ListBoxRow::new();
	row.set_widget_name(name);
	let layout = gtk::Box::new(gtk::Orientation::Horizontal, 0);
	row.add(&layout);

	// Label
	let label = gtk::Label::new(label);
	label.set_property_margin(12);
	label.set_halign(gtk::Align::Start);
	label.set_use_markup(true);
	layout.pack_start(&label, true, true, 12);

	// Icon
	let image = gtk::Image::from_icon_name(icon, gtk::IconSize::Button);
	layout.pack_end(&image, false, false, 12);

	row
}
