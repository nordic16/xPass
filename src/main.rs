mod cli;
mod utils;
mod app;

use std::error::Error;

use app::App;
use confy;
use utils::user_config::UserConfig;

fn main() -> Result<(), Box<dyn Error>> {
    let cfg: UserConfig = confy::load("xPass")?;
    let mut app = App::new();

    app.cursive.set_user_data(cfg);
    app.start_event_loop();

    let app_data = app.cursive.user_data::<UserConfig>().unwrap(); 

    let data = toml::ser::to_string(&app_data)?;
    println!("{}", data);

    confy::store("xPass", app_data)?;

    Ok(())
}