extern crate web_view;
use serde::Deserialize;
use std::sync::mpsc::Sender;
use web_view::*;
// const INDEX: &str = include_str!("ui/index.html");
const TITLE: &str = "Amplion";

pub fn main(sender: Sender<GuiMessage>) {
    web_view::builder()
        .title(TITLE)
        .content(Content::Url(format!("http://127.0.0.1:3000")))
        .size(1000, 1000)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, arg| {
            sender.send(parser(arg.to_string())).unwrap();
            Ok(())
        })
        .run()
        .unwrap();
}

#[derive(Debug, Deserialize)]
pub struct GuiMessage {
    pub cmd: String,
    pub r#type: String,
    pub value: String,
    pub id: String
}


pub fn parser(arg: String) -> GuiMessage {
    serde_json::from_str(&arg).unwrap()
}
