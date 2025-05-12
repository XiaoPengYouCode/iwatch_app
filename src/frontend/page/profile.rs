use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
struct SettingsItemProps {
    icon: String,
    label: String,
}

#[component]
pub fn ProfilePage() -> Element {
    rsx! {

        div { class: "card p-4 bg-white shadow-md rounded-lg m-3 mb-5",
            h3 { class: "font-bold text-gray-800 m-2 text-lg", "设置" }

            div { class: "flex item-center justify-between p-1 pt-3 pb-3 text-gray-600 font-semibold hover:bg-gray-100 rounded-lg",
                div { class: "flex flex-row items-center",
                    div { class: "mask mask-squircle w-12 overflow-hidden mr-2",
                        img { src: "https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp" }
                    }
                    div { class: "font-bold text-gray-800", "ID: 12345678" }
                }
                span { class: "material-icons", style: "color: #999", "chevron_right" }
            }

            SettingsItem {
                icon: "notifications".to_string(),
                label: "通知与提醒".to_string(),
            }

            SettingsItem {
                icon: "privacy_tip".to_string(),
                label: "隐私与安全".to_string(),
            }

            SettingsItem {
                icon: "help".to_string(),
                label: "帮助与反馈".to_string(),
            }
        }
    }
}

#[allow(non_snake_case)]
fn SettingsItem(prop: SettingsItemProps) -> Element {
    rsx! {
        div { class: "flex justify-between p-1 pt-3 pb-3 text-gray-600 font-semibold hover:bg-gray-100 rounded-lg",
            div { style: "display: flex; gap: 12px; align-items: center;",
                span { class: "material-icons", style: "color: #555;", "{prop.icon}" }
                span { "{prop.label}" }
            }
            span {
                class: "material-icons",
                style: "color: #999; font-size: 18px;",
                "chevron_right"
            }
        }
    }
}
