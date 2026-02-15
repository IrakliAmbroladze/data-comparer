use leptos::prelude::*;

#[component]
pub fn DiffCell(value: f64) -> impl IntoView {
    let class_name = if value < 0.0 {
        "diff-red"
    } else if value > 0.0 {
        "diff-brown"
    } else {
        "diff-green"
    };
    view! { <td class=class_name>{format!("{:.2}", value)}</td> }
}
