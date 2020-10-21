pub mod main_window;

mod page;

pub struct MainWindow {
	pub application_window: gtk::ApplicationWindow,
	header: gtk::HeaderBar,
	stack: gtk::Stack,
	switcher: gtk::StackSwitcher,
	// Back button/title
	back_button: gtk::Button,
	back_title: gtk::Label,
}
