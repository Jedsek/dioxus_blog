use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{components::*, route::Route};

pub fn Home(cx: Scope) -> Element {
    render! {
        TopBar {}
        hr {}
        Chat {}
        hr {}
        Cards {}
    }
}

fn TopBar(cx: Scope) -> Element {
    render! {
        div { class: "flex flex-row m-2",
            div { class: "grow",
                strong { r#"博客须知"# }
                br {}
                "● 点击 "
                Link { to: Route::Categories {}, span { class: "link", "分类" } }
                " 进入导航页, 查看更多文章~~"
                br {}
                "● 若为系列文章, 会使用 \"系列/p1\", \"系列/p2\" 等形式作为网址"
                br {}
                "● 敲下 ? 查看本站的快捷键 (暂无)"
                br {}
                "● 点一点聊天气泡里的头像试试~~"
                br {}
            }
            div { class: "basic-1/2 mr-20", Stats {} }
        }
    }
}

fn Chat(cx: Scope) -> Element {
    render! {
        div {
            class: "mb-2",
            ChatBubble { "欢迎大家观看我的博客! 我会在这里记录编程知识, 语录, 自己写的小说等, 一些杂七杂八的东西" }
            ChatBubble { "本站使用 rust 的 dioxus 框架构建, 将 rust 编译为 wasm 运行, css 框架为 tailwind && daisyui" }
            span { class: "opacity-60 color-slate-500 bg-slate-700 p-1 m-4 text-sm rounded-lg",
                "-> 对方揉了揉你的脑袋后下线了 :)"
            }
        }
    }
}

fn Stats(cx: Scope) -> Element {
    render! {
        div { class: "stats shadow border-2 border-indigo-200 my-2 rounded-lg text-white ease-out duration-300 hover:scale-110",
            div { class: "stat",
                div { class: "stat-title", "~> 网页总浏览数" }
                div { class: "stat-value", "9,999,999" }
                div { class: "stat-desc",
                    "~> 13% 距离上月增长, 真是完美! "
                    del { class: "hover:scale-130", "(你不会真的信了吧?)" }
                }
            }
        }
    }
}

fn Cards(cx: Scope) -> Element {
    let border = "border-2 border-indigo-500";
    render! {
        div { class: "grid grid-cols-2 grid-flow-row gap-6 my-12 mx-80",
            button { class: "{border}
                    hover:-translate-x-4 hover:-translate-y-4 hover:scale-110 ease-out duration-200",
                Card { title: "语录", i { class: "iconfont icon-yulu" } }
            }
            button { class: "{border}
                    hover:translate-x-4 hover:-translate-y-4 hover:scale-110 ease-out duration-200",
                Card { title: "2", i {class: "iconfont icon-Z-fenleidaohang" } }
            }
            button { class: "{border}
                    hover:-translate-x-4 hover:translate-y-4 hover:scale-110 ease-out duration-200",
                Card { title: "3", i {class: "iconfont icon-Z-fenleidaohang" } }
            }
            button { class: "{border}
                    hover:translate-x-4 hover:translate-y-4 hover:scale-110 ease-out duration-200",
                Card { title: "4", i {class: "iconfont icon-Z-fenleidaohang" } }
            }
        }
    }
}
