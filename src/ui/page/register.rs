use super::register_page::RegisterPage;
use crate::ui::page::page_data::PageData;
use gtk::{BoxExt, ContainerExt, ListBoxExt, StackExt, WidgetExt};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl super::Register {
	pub fn new(main_window: Rc<crate::ui::MainWindow>) -> Self {
		let mut page = Self {
			layout: gtk::Box::new(gtk::Orientation::Horizontal, 0),
			list: gtk::ListBox::new(),
			stack: gtk::Stack::new(),
			main_window,
			page_data: Rc::new(RefCell::new(HashMap::new())),
			lists: gtk::Stack::new(),
		};

		// List of items
		page.stack
			.set_transition_type(gtk::StackTransitionType::SlideUpDown);

		page.lists.add_named(&page.list, "default");
		page.lists
			.set_transition_type(gtk::StackTransitionType::SlideLeftRight);

		// "Welcome" page
		page.stack.add_named(
			super::register_page::Empty::new("Register", Some("document-edit")).widget(),
			"default",
		);

		// Only actual page for now
		page.add_row(PageData::new(
			"apartments",
			"Lägenheter",
			true,
			Box::new(super::register_page::Apartments::new()),
		));

		// Temporary filler
		let temp_pages = [
			("real_estate", "Fastigheter", true),
			("rooms", "Lokaler", true),
			("garages", "Garage/p-platser", true),
			("tenants", "Hyresgäster", true),
			("property_owner", "Fastighetsägare", false),
			("types", "Typer", false),
		];
		for temp_page in temp_pages.iter() {
			page.add_row(PageData::new(
				temp_page.0,
				temp_page.1,
				temp_page.2,
				Box::new(super::register_page::Empty::new(&temp_page.1, None)),
			));
		}

		page.create_layout();

		page
	}

	fn create_layout(&self) {
		self.layout.pack_start(&self.lists, false, false, 0);

		self.layout.pack_start(
			&gtk::Separator::new(gtk::Orientation::Vertical),
			false,
			false,
			0,
		);

		self.layout.pack_start(&self.stack, true, true, 0);

		let stack = self.stack.clone();
		let main_window = self.main_window.clone();
		let page_data = self.page_data.clone();
		let lists = self.lists.clone();

		self.list.connect_row_selected(move |_, row| {
			stack.set_visible_child_name(
				&(match row {
					Some(r) => {
						let widget_name = r.get_widget_name().to_string();

						if let Some(data) = page_data.borrow_mut().get(&widget_name) {
							main_window.set_back_title(if data.show_arrow {
								Some(&data.title)
							} else {
								None
							});
							if data.show_arrow {
								lists.set_visible_child_name(&data.name);
							}
						}

						widget_name
					}
					None => "default".to_string(),
				}),
			)
		});
	}

	fn add_row(&mut self, page_data: PageData) {
		// Row
		let row = crate::ui::widget::list_item(
			&page_data.name,
			Some(&page_data.title),
			if page_data.show_arrow {
				Some("go-next-symbolic")
			} else {
				None
			},
		);

		// Add to layout
		self.list.add(&row);
		self.stack
			.add_named(page_data.page.widget(), &page_data.name);

		// Add to lists
		if let Some(list) = page_data.page.list() {
			self.lists.add_named(list, &page_data.name);
		}

		// Att to map
		self.page_data
			.borrow_mut()
			.insert(page_data.name.to_string(), page_data);
	}

	pub fn show_list_child(&self, name: Option<&str>) {
		self.lists.set_visible_child_name(match name {
			Some(n) => n,
			None => "default",
		})
	}
}

impl super::Page for super::Register {
	fn widget(&self) -> &gtk::Widget {
		&self.layout.as_ref()
	}
}
