//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.

mod chats;
mod footer;
mod header;
mod message;
mod search;
mod tags;
pub use {chats::ChatInfo, chats::Chats, chats::MessageStatus, footer::Footer, header::Header, message::Message, message::Msg, search::Search, tags::Tags};
