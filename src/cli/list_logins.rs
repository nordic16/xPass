use cursive::{views::{Panel, Button, PaddedView, LinearLayout, ScrollView}, view::Margins, traits::{Resizable, Nameable}};
use crate::utils::user_config::UserConfig;
use super::{Screen, login_info::LoginInfoScreen};


/// Lists all existing entries.
pub struct ListLoginsScreen;

impl Screen for ListLoginsScreen {
    fn draw_window(cursive: &mut cursive::Cursive) {
        let cfg = cursive.user_data::<UserConfig>().unwrap().to_owned();
        let logins = cfg.logins.clone(); // Temporary fix.
        
        // If anyone is seeing this, please send help. I can't take this anymore.
        let view = Panel::new(PaddedView::new(Margins {left: 0, top: 1, right: 0, bottom: 2},
        ScrollView::new(LinearLayout::new(cursive::direction::Orientation::Vertical)
                .with_name("logins")))
        ).title("Logins").min_size((22,0));

        cursive.add_layer(view);

        // This code sucks.
        cursive.call_on_name("logins", move |q: &mut LinearLayout| {
            for i in logins.iter() {
                let login = i.clone(); // temporary fix xD

                q.add_child(Button::new_raw(&i.name, move |q| {
                    LoginInfoScreen::draw_window(q, &login);
                }));
            }
        });

    }
}