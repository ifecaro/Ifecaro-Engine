use crate::pages;

use dioxus::prelude::*;
// use dioxus::prelude::{Routable, rsx, fc_to_builder};
// use dioxus_router::prelude::{Routable, ToRouteSegments};
use pages::{dashboard::Dashboard, page_not_found::PageNotFound, story::Story};

#[derive(Routable, Clone, Debug)]
pub enum Route {
    #[layout(crate::layout::Layout)]
    #[route("/")]
    Story {},
    #[route("/dashboard")]
    Dashboard {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
