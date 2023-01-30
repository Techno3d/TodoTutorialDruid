use data::TodoState;
use druid::{WindowDesc, AppLauncher, theme::{BUTTON_DARK, BUTTON_LIGHT, WINDOW_BACKGROUND_COLOR}, Color};
use im::Vector;
use saver::read_stored;
use ui::ui_builder;

mod ui;
mod data;
mod saver;

fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title("My Todo App")
        .window_size((400., 400.));
    
    let stored = read_stored();
    let default_state = TodoState {
        todos: Vector::from(stored.tasks),
        ..Default::default()
    };

    AppLauncher::with_window(main_window)
        .configure_env(|env, _state| {
            env.set(BUTTON_DARK, Color::rgba8(100, 100, 120, 0));
            env.set(BUTTON_LIGHT, Color::rgba8(100, 100, 120, 100));
            env.set(WINDOW_BACKGROUND_COLOR, Color::rgba8(0, 0, 0, 100));
        })
        .launch(default_state)
        .expect("Failed to start")
}
