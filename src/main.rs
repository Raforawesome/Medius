mod app;
mod components;
use dioxus::desktop::tao::dpi::LogicalSize;
//use dioxus::prelude::*;

// SETTINGS \\
const WIDTH: f32 = 700.0;
const HEIGHT: f32 = 600.0;

// launch function
fn main() {
    dioxus::desktop::launch_cfg(app::App, |cfg| {
        cfg.with_window(|window| {
            window
                .with_title("Medius")
                .with_resizable(false)
                .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        })
    });
}
