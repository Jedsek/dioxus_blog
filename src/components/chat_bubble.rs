use dioxus::prelude::*;

use crate::components::Avatar;

#[derive(Props)]
pub struct Props<'a> {
    pub children: Element<'a>,
}

pub fn ChatBubble<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    render! {
        div { class: "chat chat-start font-medium m-2",
            Avatar { uri: "/images/avatar.jpg" }
            span { class: "chat-bubble", &cx.props.children }
        }
    }
}
