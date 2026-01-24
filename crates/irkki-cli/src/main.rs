use color_eyre::Result;

mod app;
mod chat_view;
mod start_view;
mod wizard_view;

use app::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}
