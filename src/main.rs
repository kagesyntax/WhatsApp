mod app;
mod components;
mod data;
mod route;
mod views;

pub use app::App;
pub use route::Route;

fn main() {
    dioxus::launch(App);
}
