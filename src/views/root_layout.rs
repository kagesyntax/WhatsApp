use crate::components::Footer;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn RootLayout() -> Element {
    rsx! {
        div {
            class: "h-screen flex flex-col overflow-hidden",
            div {
                class: "flex-1 flex flex-col min-h-0",
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
}
