pub mod home;
pub mod register;

mod register_page;

pub trait Page {
	fn widget(&self) -> &gtk::Widget;
}

pub struct Home {
	layout: gtk::Box,
}

pub struct Register {
	layout: gtk::Box,
	list: gtk::ListBox,
}
