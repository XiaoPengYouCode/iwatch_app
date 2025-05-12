use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
struct CardProps {
    card_title: String,
    chart_values: Vec<i32>,
    chart_value_unit: String,
}

const HOURS: [&str; 9] = [
    "6:00", "8:00", "10:00", "12:00", "14:00", "16:00", "18:00", "20:00", "22:00",
];
pub const CARD_CONTAINER_CLASS: &str = "card bg-white shadow-xl rounded-xl p-2 mt-2 mb-2";
pub const PAGE_CONTAINER_CLASS: &str = "dashboard bg-gray-100 p-4";
fn render_bar(index: usize, bar_height: f32, data: i32) -> Element {
    let time = match index {
        0 => HOURS[0],
        1 => HOURS[1],
        2 => HOURS[2],
        3 => HOURS[3],
        4 => HOURS[4],
        5 => HOURS[5],
        6 => HOURS[6],
        7 => HOURS[7],
        8 => HOURS[8],
        _ => "24:00",
    };
    let (bar_color, font_color) = if index == 8 {
        (
            // 高亮显示最后一个柱子
            "bg-gradient-to-br from-blue-400 to-blue-700",
            "text-blue-600",
        )
    } else {
        // 其他柱子使用默认颜色
        (
            "bg-gradient-to-br from-blue-100 to-blue-300",
            "text-blue-400",
        )
    };

    rsx! {
        div { class: "flex flex-col items-center flex-1",
            div {
                style: "width: 35px; height: {bar_height}px; border-radius: 3px;",
                class: "{bar_color} shadow-xs",
                div { class: "flex flex-col items-center flex-1 mt-2 text-xs {font_color} font-bold",
                    "{data}"
                }
            }
            div { class: "mt-2 text-xs text-gray-400 font-semibold", "{time}" }
        }
    }
}

#[allow(non_snake_case)]
fn ActiveChartCard(prop: CardProps) -> Element {
    let heart_rate_data = prop.chart_values.clone();
    let max_steps = *heart_rate_data.iter().max().unwrap_or(&10000) as f32;

    // 计算每个步数对应的高度值
    let heights: Vec<f32> = heart_rate_data
        .iter()
        .map(|&steps| (steps as f32 / max_steps * 120.0).max(4.0))
        .collect();

    let average_data =
        (heart_rate_data.iter().sum::<i32>() as f32 / heart_rate_data.len() as f32).round();

    rsx! {
        div { class: "{CARD_CONTAINER_CLASS}",
            div { class: "text-gray-800 text-lg font-medium mb-4 flex justify-between items-center",
                CardTitle { card_title: prop.card_title.clone() }
                ActiveChartCardAverage { average_data, unit: prop.chart_value_unit.clone() }
            }
            div { class: "flex items-end h-35 gap-1",
                for i in 0..HOURS.len() {
                    {render_bar(i, heights[i], heart_rate_data[i])}
                }
            }
        }
    }
}

#[component]
fn HealthCard() -> Element {
    rsx! {
        div { class: "{CARD_CONTAINER_CLASS}",
            CardTitle { card_title: "每日健康洞察" }
            div { class: "flex gap-3 mb-4",
                span { class: "material-icons text-blue-500 text-lg", "tips_and_updates" }
                p { class: "text-sm text-gray-700 leading-5",
                    "您已经连续5天达成运动目标，继续保持这个良好的习惯！"
                }
            }
            div { class: "flex gap-3",
                span { class: "material-icons text-purple-500 text-lg", "nightlight" }
                p { class: "text-sm text-gray-700 leading-5",
                    "昨晚深度睡眠时间较短，建议今晚提前休息，保证睡眠质量。"
                }
            }
        }
    }
}

#[component]
pub fn ActivityChartPage() -> Element {
    rsx! {
        div { class: "{PAGE_CONTAINER_CLASS}",
            ActiveChartCard {
                card_title: "心率趋势".to_string(),
                chart_values: vec![72, 75, 78, 82, 79, 76, 74, 73, 75],
                chart_value_unit: "bpm".to_string(),
            }

            ActiveChartCard {
                card_title: "血氧趋势".to_string(),
                chart_values: vec![92, 91, 97, 96, 90, 92, 95, 94, 93],
                chart_value_unit: "SpO2".to_string(),
            }

            HealthCard {}
        }
    }
}

#[component]
pub fn CardTitle(card_title: String) -> Element {
    rsx! {
        div { class: "text-gray-800 text-lg font-bold m-2", "{card_title}" }
    }
}

#[component]
fn ActiveChartCardAverage(average_data: f32, unit: String) -> Element {
    rsx! {
        div { class: "text-sm text-gray-500 font-bold mr-1", "平均: {average_data} {unit}" }
    }
}
