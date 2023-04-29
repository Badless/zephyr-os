#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;
mod login_screen;
pub use login_screen::LoginScreen;
mod toggle_switch;
mod widget;
