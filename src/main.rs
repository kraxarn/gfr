use gio::prelude::*;
use gtk::prelude::*;

mod ui;

pub const APPLICATION_ICON: Option<&str> = Some("go-home");

fn main() {
	let app = gtk::Application::new(Some("com.crow.gfr"), Default::default())
		.expect("failed to init gtk application");
	app.connect_activate(|app| {
		let window = ui::MainWindow::new(app);
		window.application_window.show_all();
		window.set_back_title(None);
	});
	app.run(&std::env::args().collect::<Vec<String>>());
}
