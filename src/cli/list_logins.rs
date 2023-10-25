use super::{login_info::LoginInfoScreen, Screen};
use crate::utils::user_config::UserConfig;
use cursive::{
    traits::{Nameable, Resizable},
    view::Margins,
    views::{Button, LinearLayout, PaddedView, Panel, ScrollView, TextView},
};

/// Lists all existing entries.
pub struct ListLoginsScreen;

impl Screen for ListLoginsScreen {
    fn draw_window(cursive: &mut cursive::Cursive) {
        let cfg = cursive.user_data::<UserConfig>().unwrap().to_owned();

        // If anyone is seeing this, please send help. I can't take this anymore.
        let view = Panel::new(PaddedView::new(
            Margins {
                left: 0,
                top: 1,
                right: 0,
                bottom: 2,
            },
            ScrollView::new(
                LinearLayout::new(cursive::direction::Orientation::Vertical).with_name("logins"),
            ),
        ))
        .title("Logins")
        .min_size((22, 0));

        // Must be added to the view tree before the line below runs.
        cursive.add_layer(view);

        // This code sucks.
        cursive.call_on_name("logins", move |q: &mut LinearLayout| {
            if !cfg.logins.is_empty() {
                cfg.logins.iter().for_each(|l| {
                    let login = l.clone();

                    q.add_child(Button::new_raw(&l.name, move |q| {
                        LoginInfoScreen::draw_window(q, &login);
                    }))
                });
            } else {
                q.add_child(
                    TextView::new("No logins!\nAdd some, and they will be listed here!")
                        .h_align(cursive::align::HAlign::Center),
                );
            }
        });
    }
}
