pub use dioxus::prelude::*;
pub use dioxus_router::prelude::*;

pub fn Categories(cx: Scope) -> Element {
    let html_text = crate::Source::get("pages/categories.md").unwrap();
    let html_text = std::str::from_utf8(html_text.data.as_ref()).unwrap();
    render! { div { dangerous_inner_html: "{html_text}" } }
}
