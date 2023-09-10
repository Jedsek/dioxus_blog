use crate::components::Card;
use dioxus::prelude::*;

#[allow(unused)]
#[inline_props]
pub fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    use_eval(cx)(r#" document.title = "404" "#).ok();

    render! {
        br {}
        Card { title: "404",
            p { class: "text-xl",
                "找不到地址, 可能是因为地址变更, 不存在, 输入错误......"
                br {}
                "尝试换个地址??"
            }
        }
    }
}
