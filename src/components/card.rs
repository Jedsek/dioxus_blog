use dioxus::prelude::*;

#[derive(Props)]
pub struct Props<'a> {
    pub title: &'a str,
    pub class: Option<&'a str>,
    pub children: Element<'a>,
}

pub fn Card<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("bg-neutral text-white rounded");
    render! {
        div { class: "card",
            div { class: class,
                div { class: "card-body",
                    h2 { class: "card-title", cx.props.title }
                    &cx.props.children
                }
            }
        }
    }
}
