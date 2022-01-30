mod cli;
mod utils;
mod app;

use std::error::Error;

use app::App;
use confy;
use utils::user_config::UserConfig;
use lazy_static::lazy_static;

lazy_static! {
    static ref CFG : UserConfig = confy::load("xPass").unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();

    app.cursive.set_user_data(&CFG);
    app.start_event_loop();

    confy::store("xPass", &*CFG)?;

    Ok(())
}