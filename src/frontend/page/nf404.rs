use dioxus::prelude::*;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center h-screen bg-gray-100 text-gray-700",
            h1 { class: "text-2xl font-bold", "Page not found" }
            p { class: "text-sm mt-2",
                "We are terribly sorry, but the page you requested doesn't exist."
            }
            pre { class: "text-red-500 mt-4", "log:\nattempted to navigate to: {route:?}" }
        }
    }
}
