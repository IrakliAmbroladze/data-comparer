use leptos::prelude::*;

#[component]
pub fn EditableGrid(dataset_name: String) -> impl IntoView {
    view! {
        <div class="editable-grid">
            <h3>{dataset_name}</h3>
        </div>
    }
}
