use data_comparer_shared::Record;
use leptos::prelude::*;

#[component]
pub fn UnmatchedTable(title: String, data: Vec<Record>) -> impl IntoView {
    let (filter_id, set_filter_id) = signal(String::new());
    let (filter_name, set_filter_name) = signal(String::new());
    let (filter_amount, set_filter_amount) = signal(String::new());

    let filtered = move || {
        let mut items = data.clone();

        let f_id = filter_id.get().to_lowercase();
        let f_name = filter_name.get().to_lowercase();
        let f_amount = filter_amount.get().to_lowercase();

        items.retain(|r| {
            (f_id.is_empty() || r.id.to_lowercase().contains(&f_id))
                && (f_name.is_empty() || r.name.to_lowercase().contains(&f_name))
                && (f_amount.is_empty() || format!("{:.2}", r.amount).contains(&f_amount))
        });

        items
    };

    view! {
        <section>
            <h3>{title}</h3>
            <table>
                <thead>
                    <tr>
                        <th>"ID"</th>
                        <th>"Name"</th>
                        <th>"Amount"</th>
                    </tr>
                    <tr class="filter-row">
                        <th>
                            <input
                                type="text"
                                placeholder="Filter ID..."
                                on:input=move |ev| set_filter_id.set(event_target_value(&ev))
                                class="column-filter"
                            />
                        </th>
                        <th>
                            <input
                                type="text"
                                placeholder="Filter Name..."
                                on:input=move |ev| { set_filter_name.set(event_target_value(&ev)) }
                                class="column-filter"
                            />
                        </th>
                        <th>
                            <input
                                type="text"
                                placeholder="Filter Amount..."
                                on:input=move |ev| {
                                    set_filter_amount.set(event_target_value(&ev))
                                }
                                class="column-filter"
                            />
                        </th>
                    </tr>
                </thead>
                <tbody>
                    {move || {
                        filtered()
                            .into_iter()
                            .map(|r| {
                                let id = r.id.clone();
                                let name = r.name.clone();
                                let amount = r.amount;

                                view! {
                                    <tr>
                                        <td>{id}</td>
                                        <td>{name}</td>
                                        <td>{format!("{:.2}", amount)}</td>
                                    </tr>
                                }
                            })
                            .collect_view()
                    }}
                </tbody>
            </table>
        </section>
    }
}
