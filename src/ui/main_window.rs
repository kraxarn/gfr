use crate::ui::page::Page;
use gtk::prelude::*;

impl super::MainWindow {
	pub fn new(gtk_app: &gtk::Application) -> Self {
		let window = Self {
			application_window: gtk::ApplicationWindow::new(gtk_app),
			header: gtk::HeaderBar::new(),
			stack: gtk::Stack::new(),
			switcher: gtk::StackSwitcher::new(),
		};
		window.init();
		window
	}

	fn init(&self) {
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
		let menu_image = gtk::Image::from_icon_name(Some("view-more"), gtk::IconSize::Menu);
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

		// Application window
		self.application_window.set_titlebar(Some(&self.header));
		self.application_window.set_default_size(1280, 720);
		self.application_window.add(&self.stack);

		// Pages
		self.add_to_stack(super::page::Home::new().widget(), "Hem");
		self.add_to_stack(super::page::Register::new().widget(), "Register");
	}

	fn add_to_stack(&self, widget: &gtk::Widget, title: &str) {
		self.stack.add_titled(widget, title, title);
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
