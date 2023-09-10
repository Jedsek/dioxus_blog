use dioxus::prelude::*;

#[inline_props]
pub fn Avatar<'a>(cx: Scope<'a>, uri: &'a str) -> Element<'a> {
    let eval = use_eval(cx);

    render! {
        img {
            class: "w-12 rounded-full avatar ease-out duration-150 hover:scale-125",
            src: "{uri}",
            onclick: move |_| {
                eval(r#" document.getElementById('my_modal_2').showModal() "#).ok();
            }
        }
        dialog { id: "my_modal_2", class: "modal",
            div { class: "flex border-2 text-white bg-slate-700",
                img { class: "w-64 rounded avatar", src: "{uri}" }
                div { class: "m-4 flex flex-col",
                    h2 { class: "font-bold text-lg", "你好! 欢迎来到我的博客" }
                    div { class: "",
                        "如果你遇见了问题, 或者博客中有错误的地方, 可以联系我哦"
                        br {}
                        br {}
                        "QQ: 2948804617"
                        br {}
                        "知乎: Jedsek"
                        br {}
                        "B站: Jedsek"
                        br {}
                    }
                    div { class: "text-medium mt-auto", "(按ESC以关闭该界面)" }
                }
            }
        }
    }
}
