mod cli;
mod utils;
mod app;

use app::App;
use confy;
use utils::user_config::UserConfig;


fn main()  {
    let cfg: UserConfig = confy::load("config").expect("bruh!!!");
    let mut app = App::new();

    app.cursive.set_user_data(cfg);
    app.start_event_loop();

    let app_data = app.cursive.user_data::<UserConfig>().unwrap(); 

    confy::store("config", app_data).expect("Bruhhh!!");
}