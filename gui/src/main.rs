extern crate web_view;

use web_view::*;

const INDEX: &str = include_str!("ui/index.html");
const TITLE: &str = "Amplion";

fn main() {
    web_view::builder()
        .title(TITLE)
        .content(Content::Html(INDEX))
        .size(1000, 1000)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
