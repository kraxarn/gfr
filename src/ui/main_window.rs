use crate::ui::page::Page;
use gtk::prelude::*;
use std::rc::Rc;

impl super::MainWindow {
	pub fn new(gtk_app: &gtk::Application) -> Rc<Self> {
		let window = Rc::new(Self {
			application_window: gtk::ApplicationWindow::new(gtk_app),
			header: gtk::HeaderBar::new(),
			stack: gtk::Stack::new(),
			switcher: gtk::StackSwitcher::new(),

			back_button: gtk::Button::new(),
			back_title: gtk::Label::new(None),
		});
		window.init(window.clone());
		window
	}

	fn init(&self, clone: Rc<Self>) {
		// Window
		self.application_window
			.set_icon_name(crate::APPLICATION_ICON);

		// Header
		self.header.set_show_close_button(true);

		// Switcher
		self.switcher.set_stack(Some(&self.stack));
		self.header.set_custom_title(Some(&self.switcher));

		// Stack
		self.stack
			.set_transition_type(gtk::StackTransitionType::SlideLeftRight);

		// Menu
		let menu_button = gtk::MenuButton::new();
		let menu_image =
			gtk::Image::from_icon_name(Some("open-menu-symbolic"), gtk::IconSize::Button);
		menu_button.set_image(Some(&menu_image));

		// Popover menu
		let popover = gtk::PopoverMenu::new();
		let popover_content = gtk::Box::new(gtk::Orientation::Vertical, 4);
		popover_content.set_property_margin(12);
		popover_content.set_visible(true);

		let about_button = gtk::ModelButton::new();
		about_button.set_property_text(Some("Om"));
		about_button.set_visible(true);
		about_button.connect_clicked(show_about_dialog);

		popover_content.add(&about_button);
		popover.add(&popover_content);
		menu_button.set_popover(Some(&popover));
		self.header.pack_end(&menu_button);

		// Go back button
		self.back_button.set_image(Some(&gtk::Image::from_icon_name(
			Some("go-previous-symbolic"),
			gtk::IconSize::Button,
		)));
		self.header.pack_start(&self.back_button);
		self.header.pack_start(&self.back_title);

		// Application window
		self.application_window.set_titlebar(Some(&self.header));
		self.application_window.set_default_size(1280, 720);
		self.application_window.add(&self.stack);

		// Pages
		let register_page = Rc::new(super::page::Register::new(clone.clone()));

		self.add_to_stack(super::page::Home::new().widget(), "home", "Hem");
		self.add_to_stack(register_page.clone().widget(), "register", "Register");

		let register_clone = register_page.clone();
		let clone_clone = clone.clone();
		self.back_button.connect_clicked(move |_| {
			register_clone.show_list_child(None);
			clone_clone.toggle_back_title(false);
		});

		self.stack
			.connect_property_visible_child_notify(move |stack| {
				if let Some(name) = stack.get_visible_child_name() {
					clone.toggle_back_title(name == "register");
				}
			});
	}

	fn add_to_stack(&self, widget: &gtk::Widget, name: &str, title: &str) {
		self.stack.add_titled(widget, name, title);
	}

	pub fn set_back_title(&self, name: Option<&str>) {
		self.back_title.set_visible(name.is_some());
		self.back_button.set_visible(name.is_some());
		if let Some(n) = name {
			self.back_title.set_markup(&format!("<b>{}</b>", n));
		}
	}

	fn toggle_back_title(&self, visible: bool) {
		let show = visible && !self.back_title.get_text().is_empty();
		self.back_title.set_visible(show);
		self.back_button.set_visible(show);
	}
}

fn show_about_dialog(_: &gtk::ModelButton) {
	let dialog = gtk::AboutDialog::new();
	dialog.set_program_name(env!("CARGO_PKG_NAME"));
	dialog.set_version(option_env!("CARGO_PKG_VERSION"));
	dialog.set_license_type(gtk::License::Gpl30);
	dialog.set_website(option_env!("CARGO_PKG_HOMEPAGE"));
	dialog.set_authors(&[env!("CARGO_PKG_AUTHORS")]);
	dialog.set_logo_icon_name(crate::APPLICATION_ICON);
	dialog.set_comments(Some("Fastighetshanterare"));

	dialog.connect_response(|about_dialog, _| {
		about_dialog.set_visible(false);
	});
	dialog.set_visible(true);
}
