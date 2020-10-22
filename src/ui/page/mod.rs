use crate::ui::page::page_data::PageData;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub mod home;
mod page_data;
pub mod register;
mod register_page;

pub trait Page {
	fn widget(&self) -> &gtk::Widget;
}

pub struct Home {
	layout: gtk::Box,
}

pub struct Register {
	main_window: Rc<crate::ui::MainWindow>,
	layout: gtk::Box,
	list: gtk::ListBox,
	stack: gtk::Stack,
	page_data: Rc<RefCell<HashMap<String, PageData>>>,
	lists: gtk::Stack,
}
