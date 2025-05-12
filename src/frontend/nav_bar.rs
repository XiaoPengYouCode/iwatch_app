use chrono::{Datelike, Local};
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn NavBar() -> Element {
    let now = Local::now();
    let date_str = now.format("%Y 年 %m 月 %d 日").to_string();
    let day_str = match now.weekday() {
        chrono::Weekday::Mon => "星期一",
        chrono::Weekday::Tue => "星期二",
        chrono::Weekday::Wed => "星期三",
        chrono::Weekday::Thu => "星期四",
        chrono::Weekday::Fri => "星期五",
        chrono::Weekday::Sat => "星期六",
        chrono::Weekday::Sun => "星期日",
    };
    rsx!(
        header { class: "navbar bg-gray-100",
            div { class: "flex flex-col flex-1",
                p { class: "text-gray-800 font-bold text-2xl mb-1", "健康伴侣" }
                p { class: "text-sm font-semibold text-gray-500", "{date_str} {day_str}" }
            }
            div { class: "flex flex-none items-center",
                button {
                    class: "btn btn-circle btn-ghost tooltip tooltip-bottom",
                    "data-tip": "通知",
                    span { class: "material-icons text-primary", "notifications" }
                }
                button {
                    class: "btn btn-circle btn-ghost tooltip tooltip-bottom",
                    "data-tip": "设置",
                    span { class: "material-icons text-primary", "settings" }
                }
                div { class: "avatar",
                    div { class: "mask mask-squircle w-6 overflow-hidden",
                        img {
                            src: "https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp",
                            alt: "用户头像",
                        }
                    }
                }
            }
        }
    )
}
