use leptos::prelude::*;

#[component]
pub fn EditableGrid(dataset_name: String) -> impl IntoView {
    view! {
        <div class="editable-grid">
            <h3>{dataset_name}</h3>
            <table class="grid-table">
                <thead>
                    <tr>
                        <th>"ID"</th>
                        <th>"Name"</th>
                        <th>"Amount"</th>
                    </tr>
                </thead>
                <tbody></tbody>
            </table>
        </div>
    }
}
