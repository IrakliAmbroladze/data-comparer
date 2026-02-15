use data_comparer_shared::ComparisonResult;
use leptos::prelude::*;

use crate::components::filterable_table::MatchedTable;

#[component]
pub fn ResultsDisplay(result: ComparisonResult) -> impl IntoView {
    let matched = result.matched.clone();
    let unmatched1 = result.unmatched_from_first.clone();
    let unmatched2 = result.unmatched_from_second.clone();

    let unmatched1_count = unmatched1.len();
    let unmatched2_count = unmatched2.len();

    let (filter_u1_id, set_filter_u1_id) = signal(String::new());
    let (filter_u1_name, set_filter_u1_name) = signal(String::new());
    let (filter_u1_amount, set_filter_u1_amount) = signal(String::new());

    let (filter_u2_id, set_filter_u2_id) = signal(String::new());
    let (filter_u2_name, set_filter_u2_name) = signal(String::new());
    let (filter_u2_amount, set_filter_u2_amount) = signal(String::new());

    let filtered_unmatched1 = move || {
        let mut data = unmatched1.clone();

        let f_id = filter_u1_id.get().to_lowercase();
        let f_name = filter_u1_name.get().to_lowercase();
        let f_amount = filter_u1_amount.get().to_lowercase();

        data.retain(|r| {
            (f_id.is_empty() || r.id.to_lowercase().contains(&f_id))
                && (f_name.is_empty() || r.name.to_lowercase().contains(&f_name))
                && (f_amount.is_empty() || format!("{:.2}", r.amount).contains(&f_amount))
        });

        data
    };

    let filtered_unmatched2 = move || {
        let mut data = unmatched2.clone();

        let f_id = filter_u2_id.get().to_lowercase();
        let f_name = filter_u2_name.get().to_lowercase();
        let f_amount = filter_u2_amount.get().to_lowercase();

        data.retain(|r| {
            (f_id.is_empty() || r.id.to_lowercase().contains(&f_id))
                && (f_name.is_empty() || r.name.to_lowercase().contains(&f_name))
                && (f_amount.is_empty() || format!("{:.2}", r.amount).contains(&f_amount))
        });

        data
    };

    view! {
        <div class="results">
            <h2>"Comparison Results"</h2>
            <MatchedTable data=matched.clone() />
            <section>
                <h3>"Unmatched from Dataset 1 (" {unmatched1_count} ")"</h3>

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
                                    on:input=move |ev| set_filter_u1_id.set(event_target_value(&ev))
                                    class="column-filter"
                                />
                            </th>
                            <th>
                                <input
                                    type="text"
                                    placeholder="Filter Name..."
                                    on:input=move |ev| {
                                        set_filter_u1_name.set(event_target_value(&ev))
                                    }
                                    class="column-filter"
                                />
                            </th>
                            <th>
                                <input
                                    type="text"
                                    placeholder="Filter Amount..."
                                    on:input=move |ev| {
                                        set_filter_u1_amount.set(event_target_value(&ev))
                                    }
                                    class="column-filter"
                                />
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        {move || {
                            filtered_unmatched1()
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

            <section>
                <h3>"Unmatched from Dataset 2 (" {unmatched2_count} ")"</h3>

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
                                    on:input=move |ev| set_filter_u2_id.set(event_target_value(&ev))
                                    class="column-filter"
                                />
                            </th>
                            <th>
                                <input
                                    type="text"
                                    placeholder="Filter Name..."
                                    on:input=move |ev| {
                                        set_filter_u2_name.set(event_target_value(&ev))
                                    }
                                    class="column-filter"
                                />
                            </th>
                            <th>
                                <input
                                    type="text"
                                    placeholder="Filter Amount..."
                                    on:input=move |ev| {
                                        set_filter_u2_amount.set(event_target_value(&ev))
                                    }
                                    class="column-filter"
                                />
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        {move || {
                            filtered_unmatched2()
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
        </div>
    }
}
