use super::route::Route;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
struct NavBarItemProps {
    icon: String,
    label: String,
    to: Route,
}

#[component]
pub fn Dock() -> Element {
    rsx! {
        nav { class: "dock bg-neutral-200 shadow-md",
            NavBarItem {
                icon: "settings".to_string(),
                label: "活动".to_string(),
                to: Route::ActivityChartPage {},
            }
            NavBarItem {
                icon: "devices".to_string(),
                label: "设备".to_string(),
                to: Route::DevicePage {},
            }
            div { class: "flex flex-col items-center",
                div { class: "avatar",
                    div { class: "mask mask-squircle w-7",
                        img { src: "https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp" }
                    }
                }
                Link { class: "dock-label text-black", to: Route::ProfilePage {}, "我的" }
            }
        }
        Outlet::<Route> {}
    }
}

#[allow(non_snake_case)]
fn NavBarItem(prop: NavBarItemProps) -> Element {
    let to = prop.to.clone();
    rsx! {
        div { class: "flex flex-col items-center",
            span { class: "material-icons text-gray-500", "{prop.icon}" }
            Link { class: "dock-label text-extrabold text-gray-800", to, "{prop.label}" }
        }
    }
}
