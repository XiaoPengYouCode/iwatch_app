use dioxus::desktop::wry::dpi::{Position, Size};
use dioxus::desktop::{Config, LogicalPosition, LogicalSize, WindowBuilder};
use dioxus::prelude::*;

pub mod backend;
pub mod frontend;

use frontend::nav_bar::NavBar;
use frontend::route::Route;

fn main() {
    let window_config = Config::new().with_window(
        WindowBuilder::new()
            .with_title("Dioxus App")
            .with_resizable(false)
            .with_inner_size(Size::Logical(LogicalSize::new(420.0, 800.0)))
            .with_position(Position::Logical(LogicalPosition::new(2000.0, 0.0)))
            .with_decorations(false)
            .with_always_on_top(true),
    );
    dioxus::LaunchBuilder::desktop()
        .with_cfg(window_config)
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/icon?family=Material+Icons",
        }
        div { class: "scrollbar-hide flex flex-col h-screen bg-gray-100 font-gray-800",
            NavBar {}
            Router::<Route> {}
        }
    }
}
