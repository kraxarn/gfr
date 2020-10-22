mod apartments;
mod empty;

pub struct Apartments {
	layout: gtk::Box,
	base_options: gtk::Grid,
	list: gtk::ListBox,
}

pub struct Empty {
	layout: gtk::Box,
}

pub trait RegisterPage {
	fn widget(&self) -> &gtk::Widget;
	fn list(&self) -> Option<&gtk::ListBox>;
}
