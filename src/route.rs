use crate::views::{Calls, Chat, Communities, Home, RootLayout, Updates};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(RootLayout)]
        #[route("/")]
        Home {},
        #[route("/updates")]
        Updates {},
        #[route("/communities")]
        Communities {},
        #[route("/calls")]
        Calls {},
    #[end_layout]
    #[route("/chat/:index")]
    Chat { index: usize },
}
