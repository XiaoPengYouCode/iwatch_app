use dioxus::prelude::*;

use super::dock::Dock;
use super::page::active::ActivityChartPage;
use super::page::device::DevicePage;
use super::page::nf404::PageNotFound;
use super::page::profile::ProfilePage;

#[rustfmt::skip]
#[derive(Routable, Clone, Debug, PartialEq)]
pub enum Route {
    #[layout(Dock)]
        #[redirect("/", || Route::ActivityChartPage {})]
        #[route("/active")]
        ActivityChartPage {},
        #[route("/device")]
        DevicePage {},
        #[route("/profile")]
        ProfilePage {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
