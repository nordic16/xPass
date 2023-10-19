use crate::{
    cli::{
        create_login::CreateLoginScreen, generate_password::GeneratePasswordScreen, list_logins::ListLoginsScreen, Screen
    }, utils::{construct_dialog, user_config::UserConfig}, DEBUG
};
use cursive::{
    direction::Orientation, event::Key, theme::{
        Color, PaletteColor::{Background, Primary, Shadow, View}, Theme
    }, traits::{Nameable, Resizable}, views::{Button, EditView, LinearLayout, ListView, Panel, TextView, ViewRef}, Cursive, CursiveExt
};

pub struct App {
    pub cursive: Cursive,
}

impl App {
    pub fn new() -> Self {
        let mut cursive = Cursive::default();
        let mut theme = Theme::default();

        theme.palette[Background] = Color::TerminalDefault;
        theme.palette[Shadow] = Color::TerminalDefault;
        theme.palette[View] = Color::TerminalDefault;
        theme.palette[Primary] = Color::TerminalDefault;

        cursive.set_theme(theme);

        App { cursive }
    }

    /// Starts event loop and draws the main menu.
    pub fn start_event_loop(&mut self) {
        let app_data = self.cursive.user_data::<UserConfig>().unwrap();

        // Unset password: Must set a new one.
        if app_data.master_password.is_empty() {
            App::draw_masterpassword_screen(&mut self.cursive);
        } else if !DEBUG {
            // in case i'm debugging.
            App::draw_login_screen(&mut self.cursive);
        } else {
            App::draw_main_menu(&mut self.cursive);
        }

        self.cursive.run();
    }

    // Draws the screen that allows the user to login.
    fn draw_login_screen(cursive: &mut Cursive) {
        let content = ListView::new()
            .child("Password", EditView::new().secret().with_name("password"))
            .min_width(25);

        let action = |x: &mut Cursive| {
            let mut password: ViewRef<EditView> = x.find_name("password").unwrap();
            let cfg = x.user_data::<UserConfig>().unwrap();

            // If password matches the one previously set by the user draw the main menu screen and
            // remove everything else.
            if password.get_content().to_string() == cfg.master_password {
                x.add_layer(
                    construct_dialog("Success!", TextView::new("Login successful!")).button(
                        "Ok",
                        |x| {
                            x.pop_layer();
                            x.pop_layer();

                            App::draw_main_menu(x);
                        },
                    ),
                );
            } else {
                password.set_content("");

                x.add_layer(
                    construct_dialog(
                        "Password is wrong!",
                        TextView::new("Check your password and try again."),
                    )
                    .dismiss_button("Ok"),
                );
            }
        };

        cursive.add_layer(
            construct_dialog("Enter master password.", content).button("Confirm", action),
        );
    }

    fn draw_main_menu(cursive: &mut Cursive) {
        let view = Panel::new(
            LinearLayout::new(Orientation::Vertical)
                .child(Button::new_raw("Create new password", |x| {
                    CreateLoginScreen::draw_window(x);
                }))
                .child(Button::new_raw("List passwords", |x| {
                    ListLoginsScreen::draw_window(x);
                }))
                .child(Button::new_raw("Generate password", |x| {
                    GeneratePasswordScreen::draw_window(x);
                }))
                .child(Button::new_raw("Clear passwords", |q| {
                    q.with_user_data(|cfg: &mut UserConfig| cfg.logins.clear());
                    q.add_layer(
                        construct_dialog("Success!", TextView::new("Passwords cleared!"))
                            .dismiss_button("Ok"),
                    );
                })),
        )
        .title("xPass");

        // Global callbacks.
        cursive.add_global_callback('q', |x| x.quit());
        cursive.add_global_callback(Key::Esc, |x| {
            // Prevents the user from closing the main menu.
            if x.screen().len() > 1 {
                x.pop_layer();
            }
        });

        cursive.add_layer(view);
    }

    /// Draws the screen that lets the user set the master password.
    fn draw_masterpassword_screen(cursive: &mut Cursive) {
        let content = ListView::new()
            .child("Password", EditView::new().secret().with_name("password"))
            .child(
                "Confirm Password",
                EditView::new().secret().with_name("confirm_password"),
            )
            .min_width(35);

        // Closure that will be executed when the user presses ok.
        let action = |x: &mut Cursive| {
            let password: ViewRef<EditView> = x.find_name("password").unwrap();
            let c_password: ViewRef<EditView> = x.find_name("confirm_password").unwrap();

            if password.get_content() == c_password.get_content()
                && !password.get_content().is_empty()
            {
                x.add_layer(
                    construct_dialog("Success!", TextView::new("Password set!")).button(
                        "Ok",
                        |x| {
                            x.pop_layer();
                            x.pop_layer();

                            App::draw_main_menu(x);
                        },
                    ),
                );

                // Actually sets the password.
                x.with_user_data(|cfg: &mut UserConfig| {
                    cfg.master_password = password.get_content().to_string()
                });

            // Invalid password.
            } else {
                x.add_layer(
                    construct_dialog("Error.", TextView::new("Make sure your passwords match!"))
                        .dismiss_button("Ok"),
                );
            }
        };
        cursive.add_layer(
            construct_dialog("Set a master password.", content).button("Confirm", action),
        );
    }
}
