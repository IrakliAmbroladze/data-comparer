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

    let update_cell = move |row_idx: usize, field: String, value: String| {
        set_grid_data.update(|data| {
            if let Some(record) = data.get_mut(row_idx) {
                match field.as_str() {
                    "id" => record.id = value,
                    "name" => record.name = value,
                    "amount" => {
                        if let Ok(amt) = value.parse::<f64>() {
                            record.amount = amt;
                        }
                    }
                    _ => {}
                }
            }
        });
    };

    let add_row = move |_| {
        set_grid_data.update(|data| {
            data.push(Record::new(String::new(), String::new(), 0.0));
        });
    };

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
                    <For
                        each=move || grid_data.get().into_iter().enumerate()
                        key=|(idx, _)| *idx
                        children=move |(row_idx, record)| {
                            let id_val = record.id.clone();
                            let name_val = record.name.clone();
                            let amount_val = record.amount;
                            view! {
                                <tr>
                                    <td>
                                        <input
                                            type="text"
                                            value=id_val
                                            on:input=move |ev| {
                                                update_cell(
                                                    row_idx,
                                                    "id".to_string(),
                                                    event_target_value(&ev),
                                                );
                                            }
                                        />
                                    </td>
                                    <td>
                                        <input
                                            type="text"
                                            value=name_val
                                            on:input=move |ev| {
                                                update_cell(
                                                    row_idx,
                                                    "name".to_string(),
                                                    event_target_value(&ev),
                                                );
                                            }
                                        />
                                    </td>
                                    <td>
                                        <input
                                            type="number"
                                            step="0.01"
                                            value=amount_val
                                            on:input=move |ev| {
                                                update_cell(
                                                    row_idx,
                                                    "amount".to_string(),
                                                    event_target_value(&ev),
                                                );
                                            }
                                        />
                                    </td>
                                </tr>
                            }
                        }
                    />
                </tbody>
            </table>
            <button on:click=add_row class="add-row-btn">
                "+ Add Row"
            </button>
        </div>
    }
}
