mod ui;
mod utils;
mod login;
mod app;

use app::App;
use ui::main_screen::MainScreen;

use crate::login::Login;

fn main() {

    let logins = utils::retrieve_from_datafile();
    let mut app = App::new(&logins);
    
    app.cursive.set_user_data(app.logins);
    MainScreen::start_event_loop(&mut app.cursive);

    // Retrieves all login entries after the user decides to close the app.
    let logins = app.cursive.user_data::<Vec<Login>>().unwrap();
    println!("{:?}", logins);

    utils::write_to_datafile(logins).expect("failed to write to datafile.");

}
