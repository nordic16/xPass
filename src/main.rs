mod app;
mod cli;
mod utils;

#[cfg(test)]
mod tests;

use std::error::Error;

use app::App;
use utils::user_config::UserConfig;

// disables password authentication
static DEBUG: bool = true;

fn main() -> Result<(), Box<dyn Error>> {
    let cfg: UserConfig = confy::load("xPass")?;
    let mut app = App::new();

    app.cursive.set_user_data(cfg);
    app.start_event_loop();

    let app_data = app.cursive.user_data::<UserConfig>().unwrap();

    confy::store("xPass", app_data)?;

    Ok(())
}
