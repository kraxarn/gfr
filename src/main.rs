use gio::prelude::*;
use gtk::prelude::*;

mod data;
mod ui;

pub const APPLICATION_ICON: Option<&str> = Some("go-home");

fn init_logger() -> Result<(), simplelog::TermLogError> {
	simplelog::TermLogger::init(
		simplelog::LevelFilter::max(),
		simplelog::Config::default(),
		simplelog::TerminalMode::Mixed,
	)
}

fn main() {
	init_logger().expect("failed to init logger");

	let app = gtk::Application::new(Some("com.crow.gfr"), Default::default())
		.expect("failed to init gtk application");
	app.connect_activate(|app| {
		let window = ui::MainWindow::new(app);
		window.application_window.show_all();
		window.set_back_title(None);
	});
	app.run(&std::env::args().collect::<Vec<String>>());
}
