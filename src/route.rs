use crate::components::*;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/categories")]
    Categories {},
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
