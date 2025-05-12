use dioxus::prelude::*;
use futures_util::StreamExt;
use tracing::info;

use crate::backend::ble::DeviceManager;
use crate::page::active::{CardTitle, CARD_CONTAINER_CLASS, PAGE_CONTAINER_CLASS};

enum BleUpdate {
    Connect,
    Disconnect,
}

#[allow(non_snake_case)]
pub fn DevicePage() -> Element {
    const DEVICE_IMG: Asset = asset!("/assets/icon/image-removebg-preview.png");
    let mut is_connected = use_signal(|| false);
    let profile = use_coroutine(|mut rx: UnboundedReceiver<BleUpdate>| async move {
        let mut device_manager = DeviceManager::new().await.unwrap();
        while let Some(msg) = rx.next().await {
            match msg {
                BleUpdate::Connect => device_manager.scan_and_connect_ble().await.unwrap(),
                BleUpdate::Disconnect => device_manager.disconnect_ble().await.unwrap(),
            }
        }
    });

    rsx! {
        div { class: "{PAGE_CONTAINER_CLASS} space-y-6",
            div { class: "{CARD_CONTAINER_CLASS} flex flex-row items-center bg-base-100 shadow-md rounded-lg p-4",
                img { src: DEVICE_IMG, class: "w-24 h-24 rounded-full" }
                div { class: "flex flex-col flex-1 ml-4",
                    div { class: "text-lg font-bold text-gray-800", "我的智能手表" }
                    if is_connected() {
                        div { class: "text-sm text-gray-600", "已连接 • 电量 100%" }
                    } else {
                        div { class: "text-sm text-gray-600", "未连接" }
                    }
                }
                button {
                    class: "btn btn-sm mr-1",
                    class: if !is_connected() { "btn-primary" },
                    onclick: move |_| {
                        profile.send(BleUpdate::Connect);
                        info!("Click connect button, sending connect command");
                        is_connected.set(!is_connected());
                    },
                    disabled: is_connected(),
                    "连接"
                }
                button {
                    class: "btn btn-sm",
                    class: if is_connected() { "btn-primary" },
                    onclick: move |_| {
                        profile.send(BleUpdate::Disconnect);
                        info!("Click connect button, sending connect command");
                        is_connected.set(!is_connected());
                    },
                    disabled: !is_connected(),
                    "断开"
                }
            }
            DeviceConfigCard {}
        }
    }
}

#[component]
fn DeviceConfigCard() -> Element {
    rsx! {
        div { class: "card bg-white shadow-md rounded-lg p-6",
            CardTitle { card_title: "功能与服务" }

            div { class: "grid grid-cols-3 gap-4",

                DeviceConfigCardItem {
                    icon: "update".to_string(),
                    label: "固件更新".to_string(),
                }

                DeviceConfigCardItem {
                    icon: "watch".to_string(),
                    label: "表盘商店".to_string(),
                }

                DeviceConfigCardItem {
                    icon: "notifications_active".to_string(),
                    label: "通知设置".to_string(),
                }

                DeviceConfigCardItem { icon: "alarm".to_string(), label: "闹钟".to_string() }

                DeviceConfigCardItem {
                    icon: "more_horiz".to_string(),
                    label: "更多".to_string(),
                }
            }
        }
    }
}

#[component]
fn DeviceConfigCardItem(icon: String, label: String) -> Element {
    rsx! {
        div { class: "flex flex-col items-center text-center cursor-pointer transition-transform transform hover:scale-105 hover:text-blue-500",
            span { class: "material-icons text-blue-400 text-3xl mb-2", "{icon}" }
            span { class: "text-sm text-gray-600 hover:text-blue-500", "{label}" }
        }
    }
}
