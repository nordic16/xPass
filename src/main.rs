mod ui;
mod utils;
mod login;
mod app;

use app::App;

use crate::login::Login;

fn main() {

    let logins = utils::retrieve_from_datafile();
    let mut app = App::new();
    
    app.cursive.set_user_data(logins);
    app.start_event_loop();

    // Retrieves all login entries after the user decides to close the app.
    let logins = app.cursive.user_data::<Vec<Login>>().unwrap();
    println!("{:?}", logins);

    utils::write_to_datafile(logins).expect("failed to write to datafile.");

}
