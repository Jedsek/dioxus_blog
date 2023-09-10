#![allow(non_snake_case)]
// #![allow(non_upper_case_globals)]

mod components;
mod markdown;
mod route;
mod utils;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use route::Route;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "dist/"]
pub struct Source;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    render! {
        div { Router::<Route> {} }
    }
}
