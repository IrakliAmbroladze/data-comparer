use leptos::prelude::*;

#[component]
pub fn ColumnFilter(placeholder: String, on_change: Callback<String>) -> impl IntoView {
    view! {
        <input
            type="text"
            placeholder=placeholder
            on:input=move |ev| on_change.run(event_target_value(&ev))
            class="column-filter"
        />
    }
}

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
