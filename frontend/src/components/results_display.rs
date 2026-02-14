use data_comparer_shared::ComparisonResult;
use leptos::{logging::log, prelude::*};

#[component]
pub fn ResultsDisplay(result: ComparisonResult) -> impl IntoView {
    let matched = result.matched.clone();
    let matched_count = matched.len();

    let (filter_id, set_filter_id) = signal(String::new());
    let (filter_name1, set_filter_name1) = signal(String::new());
    let (filter_amount1, set_filter_amount1) = signal(String::new());
    let (filter_name2, set_filter_name2) = signal(String::new());
    let (filter_amount2, set_filter_amount2) = signal(String::new());

    let toggle_sort = move |column: &'static str| {
        move |_| {
            log!("clicked on {}", column);
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
                        {result
                            .matched
                            .iter()
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
                                        <td class=if diff < 0.0 {
                                            "diff-red"
                                        } else {
                                            "diff-green"
                                        }>{format!("{:.2}", diff)}</td>
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </tbody>
                </table>
            </section>

            <section>
                <h3>"Unmatched from Dataset 1 (" {result.unmatched_from_first.len()} ")"</h3>
                <table>
                    <thead>
                        <tr>
                            <th>"ID"</th>
                            <th>"Name"</th>
                            <th>"Amount"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {result
                            .unmatched_from_first
                            .iter()
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
                            .collect_view()}
                    </tbody>
                </table>
            </section>

            <section>
                <h3>"Unmatched from Dataset 2 (" {result.unmatched_from_second.len()} ")"</h3>
                <table>
                    <thead>
                        <tr>
                            <th>"ID"</th>
                            <th>"Name"</th>
                            <th>"Amount"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {result
                            .unmatched_from_second
                            .iter()
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
                            .collect_view()}
                    </tbody>
                </table>
            </section>
        </div>
    }
}
