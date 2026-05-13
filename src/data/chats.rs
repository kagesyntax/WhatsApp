use crate::components::{ChatInfo, MessageStatus};
use dioxus::prelude::*;

pub fn get_chats() -> Vec<ChatInfo> {
    Vec::from([
        ChatInfo {
            name: "Praise".to_string(),
            message: "Hello, This is Praise speaking - over.".to_string(),
            time: "10:20 pm".to_string(),
            status: MessageStatus::Read,
            image: None,
        },
        ChatInfo {
            name: "Liang Wenfeng".to_string(),
            message: "DeepSeek-R2 is going to blow your mind.".to_string(),
            time: "9:45 pm".to_string(),
            status: MessageStatus::Read,
            image: Some(asset!("/assets/users/liang-wenfeng.jpg")),
        },
        ChatInfo {
            name: "Yang Zhilin".to_string(),
            message: "Kimi K2.5 just dropped — open-source agent swarms.".to_string(),
            time: "8:30 pm".to_string(),
            status: MessageStatus::Read,
            image: None,
        },
        ChatInfo {
            name: "Zhang Peng".to_string(),
            message: "GLM-5 just hit 90% on MMLU.".to_string(),
            time: "7:15 pm".to_string(),
            status: MessageStatus::Delivered,
            image: None,
        },
        ChatInfo {
            name: "Sam Altman".to_string(),
            message: "AGI is closer than you think.".to_string(),
            time: "6:00 pm".to_string(),
            status: MessageStatus::Delivered,
            image: Some(asset!("/assets/users/sam-altman.jpg")),
        },
        ChatInfo {
            name: "Mark Zuckerberg".to_string(),
            message: "Meta just open-sourced Llama 4.".to_string(),
            time: "4:20 pm".to_string(),
            status: MessageStatus::Sent,
            image: Some(asset!("/assets/users/mark-zuckerberg.jpg")),
        },
        ChatInfo {
            name: "Elon Musk".to_string(),
            message: "xAI's Grok-3 is training right now.".to_string(),
            time: "2:06 pm".to_string(),
            status: MessageStatus::Sent,
            image: Some(asset!("/assets/users/elon-musk.jpg")),
        },
        ChatInfo {
            name: "Jensen Huang".to_string(),
            message: "Blackwell GPUs are shipping next week.".to_string(),
            time: "1:30 pm".to_string(),
            status: MessageStatus::Read,
            image: Some(asset!("/assets/users/jensen-huang.jpg")),
        },
        ChatInfo {
            name: "Demis Hassabis".to_string(),
            message: "AlphaFold 3 is now production-ready.".to_string(),
            time: "11:00 am".to_string(),
            status: MessageStatus::Delivered,
            image: Some(asset!("/assets/users/demis-hassabis.jpg")),
        },
        ChatInfo {
            name: "Jeff Bezos".to_string(),
            message: "Blue Origin is hiring AI engineers.".to_string(),
            time: "9:20 am".to_string(),
            status: MessageStatus::Read,
            image: Some(asset!("/assets/users/jeff-bezos.jpg")),
        },
    ])
}
