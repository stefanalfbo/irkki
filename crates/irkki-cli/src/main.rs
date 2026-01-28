use color_eyre::Result;

mod app;
mod chat_view;
mod start_view;
mod widget;
mod wizard_view;

use app::App;
use flexi_logger::{FileSpec, Logger};

fn main() -> Result<()> {
    Logger::try_with_env()?
        .log_to_file(FileSpec::default())
        .start()?;

    log::info!("Starting irkki-cli application");

    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}
