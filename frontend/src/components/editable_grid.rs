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
                <tbody>
                    <tr>
                        <td>
                            <input type="text" value=1 />
                        </td>
                        <td>
                            <input type="text" value="Irakli" />
                        </td>
                        <td>
                            <input type="number" step="0.01" value=120 />
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}
