mod cli;
mod utils;
mod app;

use app::App;
use confy;
use utils::user_config::UserConfig;

fn main()  {
    let cfg: UserConfig = confy::load("xPass").expect("bruh!!!");
    let mut app = App::new();

    app.cursive.set_user_data(cfg);
    app.start_event_loop();

    let app_data = app.cursive.user_data::<UserConfig>().unwrap(); 

    confy::store("xPass", app_data).expect("Bruhhh!!");
}