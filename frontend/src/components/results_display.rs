use data_comparer_shared::ComparisonResult;
use leptos::prelude::*;

#[component]
pub fn ResultsDisplay(result: ComparisonResult) -> impl IntoView {
    let matched = result.matched.clone();
    let unmatched1 = result.unmatched_from_first.clone();
    let unmatched2 = result.unmatched_from_second.clone();

    let matched_count = matched.len();
    let unmatched1_count = unmatched1.len();
    let unmatched2_count = unmatched2.len();

    let (filter_id, set_filter_id) = signal(String::new());
    let (filter_name1, set_filter_name1) = signal(String::new());
    let (filter_amount1, set_filter_amount1) = signal(String::new());
    let (filter_name2, set_filter_name2) = signal(String::new());
    let (filter_amount2, set_filter_amount2) = signal(String::new());

    let (filter_u1_id, set_filter_u1_id) = signal(String::new());
    let (filter_u1_name, set_filter_u1_name) = signal(String::new());
    let (filter_u1_amount, set_filter_u1_amount) = signal(String::new());

    let (filter_u2_id, set_filter_u2_id) = signal(String::new());
    let (filter_u2_name, set_filter_u2_name) = signal(String::new());
    let (filter_u2_amount, set_filter_u2_amount) = signal(String::new());

    let (matched_sort, set_matched_sort) = signal("id".to_string());
    let (matched_sort_desc, set_matched_sort_desc) = signal(false);

    let filtered_matched = move || {
        let mut data = matched.clone();

        let f_id = filter_id.get().to_lowercase();
        let f_name1 = filter_name1.get().to_lowercase();
        let f_amount1 = filter_amount1.get().to_lowercase();
        let f_name2 = filter_name2.get().to_lowercase();
        let f_amount2 = filter_amount2.get().to_lowercase();

        data.retain(|r| {
            (f_id.is_empty() || r.id.to_lowercase().contains(&f_id))
                && (f_name1.is_empty() || r.first_name.to_lowercase().contains(&f_name1))
                && (f_amount1.is_empty() || format!("{:.2}", r.first_amount).contains(&f_amount1))
                && (f_name2.is_empty() || r.second_name.to_lowercase().contains(&f_name2))
                && (f_amount2.is_empty() || format!("{:.2}", r.second_amount).contains(&f_amount2))
        });

        let sort_by = matched_sort.get();
        let desc = matched_sort_desc.get();
        data.sort_by(|a, b| {
            let cmp = match sort_by.as_str() {
                "id" => a.id.cmp(&b.id),
                "name" => a.first_name.cmp(&b.first_name),
                "amount1" => a.first_amount.partial_cmp(&b.first_amount).unwrap(),
                "amount2" => a.second_amount.partial_cmp(&b.second_amount).unwrap(),
                "diff" => a
                    .amount_difference
                    .abs()
                    .partial_cmp(&b.amount_difference.abs())
                    .unwrap(),
                _ => std::cmp::Ordering::Equal,
            };
            if desc {
                cmp.reverse()
            } else {
                cmp
            }
        });

        data
    };

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

    let toggle_sort = move |column: &'static str| {
        move |_| {
            if matched_sort.get() == column {
                set_matched_sort_desc.update(|d| *d = !*d);
            } else {
                set_matched_sort.set(column.to_string());
                set_matched_sort_desc.set(false);
            }
        }
    };
    view! {
        <div class="results">
            <h2>"Comparison Results"</h2>

            <section>
                <h3>"Matched Records (" {matched_count} ")"</h3>
                <table>
                    <thead>
                        <tr>
                            <th on:click=toggle_sort("id") class="sortable">
                                "ID"
                            </th>
                            <th on:click=toggle_sort("name") class="sortable">
                                "Name (Dataset 1)"
                            </th>
                            <th on:click=toggle_sort("amount1") class="sortable">
                                "Amount (Dataset 1)"
                            </th>
                            <th>"Name (Dataset 2)"</th>
                            <th on:click=toggle_sort("amount2") class="sortable">
                                "Amount (Dataset 2)"
                            </th>
                            <th on:click=toggle_sort("diff") class="sortable">
                                "Difference"
                            </th>
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
                                    on:input=move |ev| set_filter_name1.set(event_target_value(&ev))
                                    class="column-filter"
                                />
                            </th>
                            <th>
                                <input
                                    type="text"
                                    placeholder="Filter Amount..."
                                    on:input=move |ev| {
                                        set_filter_amount1.set(event_target_value(&ev))
                                    }
                                    class="column-filter"
                                />
                            </th>
                            <th>
                                <input
                                    type="text"
                                    placeholder="Filter Name..."
                                    on:input=move |ev| set_filter_name2.set(event_target_value(&ev))
                                    class="column-filter"
                                />
                            </th>
                            <th>
                                <input
                                    type="text"
                                    placeholder="Filter Amount..."
                                    on:input=move |ev| {
                                        set_filter_amount2.set(event_target_value(&ev))
                                    }
                                    class="column-filter"
                                />
                            </th>
                            <th></th>
                        </tr>
                    </thead>
                    <tbody>
                        {move || {
                            filtered_matched()
                                .into_iter()
                                .map(|m| {
                                    let id = m.id.clone();
                                    let first_name = m.first_name.clone();
                                    let first_amount = m.first_amount;
                                    let second_name = m.second_name.clone();
                                    let second_amount = m.second_amount;
                                    let diff = m.amount_difference;
                                    view! {
                                        <tr>
                                            <td>{id}</td>
                                            <td>{first_name}</td>
                                            <td>{format!("{:.2}", first_amount)}</td>
                                            <td>{second_name}</td>
                                            <td>{format!("{:.2}", second_amount)}</td>
                                            <td class=move || {
                                                if diff < 0.0 {
                                                    "diff-red"
                                                } else if diff > 0.0 {
                                                    "diff-brown"
                                                } else {
                                                    "diff-green"
                                                }
                                            }>{format!("{:.2}", diff)}</td>
                                        </tr>
                                    }
                                })
                                .collect_view()
                        }}
                    </tbody>
                </table>
            </section>

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
