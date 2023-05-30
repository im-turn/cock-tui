use cock_lib::bin_use::{
    UserData,
};
use cock_tui::draw::{popup_theme_menu, TUIDisplay};
use cursive::event::Key;

fn main() {
    // init cursive runnable instance
    let mut siv = cursive::default();

    // set `UserData` struct as user data for cursive instance
    siv.set_user_data(UserData::default());

    // global callback to exit the program
    siv.add_global_callback(Key::Esc, |s| s.quit());

    // global callback to change the theme
    siv.add_global_callback('\\', |s| popup_theme_menu(s));

    // get the initial state set by `UserData::default()`
    let state = siv.user_data::<UserData>().unwrap().clone().state;

    // begin drawing the screens from default state
    state.draw(&mut siv);

    // run cursive instance
    siv.run();
}
