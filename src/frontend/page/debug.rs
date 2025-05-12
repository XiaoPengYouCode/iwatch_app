use std::sync::{Arc, Mutex};

use dioxus::prelude::*;
use tracing::error;

use crate::comm::ble::ble_device::DeviceManager;

#[component]
pub fn DebugPage() -> Element {
    // Create a signal to hold the DeviceManager instance
    let mut device_manager = use_signal::<Option<Arc<Mutex<DeviceManager>>>>(|| None);
    // Create a signal for scan status or error messages
    let mut scan_status = use_signal(|| String::from("Ready to scan"));

    // Initialize DeviceManager when the component mounts
    use_effect(move || {
        spawn(async move {
            match DeviceManager::new().await {
                Ok(dm) => {
                    device_manager.set(Some(Arc::new(Mutex::new(dm))));
                    scan_status.set("DeviceManager initialized".to_string());
                }
                Err(e) => {
                    error!("Failed to initialize DeviceManager: {}", e);
                    scan_status.set(format!("Error: {}", e));
                }
            }
        });
    });

    // connect
    let connect_device = move |_| {
        spawn(async move {
            if let Some(dm) = device_manager.read().as_ref() {
                let mut dm = dm.lock().unwrap();
                match dm.scan_ble().await {
                    Ok(_) => scan_status.set("Scan completed successfully".to_string()),
                    Err(e) => {
                        error!("Scan failed: {}", e);
                        scan_status.set(format!("Scan error: {}", e));
                    }
                }
            } else {
                scan_status.set("DeviceManager not initialized".to_string());
            }
        });
    };

    rsx! {
        div {
            div { class: "text-gray-800", "Bluetooth Scanner" }
            button {
                class: "button bg-blue-500 text-white p-2 rounded",
                onclick: connect_device,
                "Scan Devices"
            }
        }
    }
}
