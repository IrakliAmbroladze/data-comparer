use data_comparer_shared::Record;
use leptos::prelude::*;

#[component]
pub fn EditableGrid(
    dataset_name: String,
    initial_data: Signal<Vec<Record>>,
    on_data_change: Callback<Vec<Record>>,
) -> impl IntoView {
    let (grid_data, set_grid_data) = signal(Vec::<Record>::new());

    Effect::new(move |_| {
        let data = initial_data.get();
        if !data.is_empty() {
            set_grid_data.set(data);
        }
    });

    Effect::new(move |_| {
        let data = grid_data.get();
        on_data_change.run(data);
    });
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
