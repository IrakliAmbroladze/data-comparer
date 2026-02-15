use crate::components::filterable_table::{ColumnFilter, DiffCell};
use data_comparer_shared::MatchedRecord;
use leptos::prelude::*;

#[component]
pub fn MatchedTable(data: Vec<MatchedRecord>) -> impl IntoView {
    let matched_count = data.len();
    let (filter_id, set_filter_id) = signal(String::new());
    let (filter_name1, set_filter_name1) = signal(String::new());
    let (filter_amount1, set_filter_amount1) = signal(String::new());
    let (filter_name2, set_filter_name2) = signal(String::new());
    let (filter_amount2, set_filter_amount2) = signal(String::new());
    let (filter_diff, set_filter_diff) = signal(String::new());

    let (matched_sort, set_matched_sort) = signal("id".to_string());
    let (matched_sort_desc, set_matched_sort_desc) = signal(false);

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
    let filtered = move || {
        let mut items = data.clone();

        let f_id = filter_id.get().to_lowercase();
        let f_name1 = filter_name1.get().to_lowercase();
        let f_amount1 = filter_amount1.get().to_lowercase();
        let f_name2 = filter_name2.get().to_lowercase();
        let f_amount2 = filter_amount2.get().to_lowercase();
        let f_diff = filter_diff.get().to_lowercase();

        items.retain(|r| {
            (f_id.is_empty() || r.id.to_lowercase().contains(&f_id))
                && (f_name1.is_empty() || r.first_name.to_lowercase().contains(&f_name1))
                && (f_amount1.is_empty() || format!("{:.2}", r.first_amount).contains(&f_amount1))
                && (f_name2.is_empty() || r.second_name.to_lowercase().contains(&f_name2))
                && (f_amount2.is_empty() || format!("{:.2}", r.second_amount).contains(&f_amount2))
                && (f_diff.is_empty() || format!("{:.2}", r.amount_difference).contains(&f_diff))
        });

        let sort_by = matched_sort.get();
        let desc = matched_sort_desc.get();

        items.sort_by(|a, b| {
            let cmp = match sort_by.as_str() {
                "id" => a.id.cmp(&b.id),
                "name1" => a.first_name.cmp(&b.first_name),
                "amount1" => a.first_amount.partial_cmp(&b.first_amount).unwrap(),
                "name2" => a.second_name.cmp(&b.second_name),
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

        items
    };

    view! {
        <section>
            <h3>"Matched Records (" {matched_count} ")"</h3>
            <table>
                <thead>
                    <tr>
                        <th on:click=toggle_sort("id") class="sortable">
                            "ID"
                        </th>
                        <th on:click=toggle_sort("name1") class="sortable">
                            "Name (Dataset 1)"
                        </th>
                        <th on:click=toggle_sort("amount1") class="sortable">
                            "Amount (Dataset 1)"
                        </th>
                        <th on:click=toggle_sort("name2") class="sortable">
                            "Name (Dataset 2)"
                        </th>
                        <th on:click=toggle_sort("amount2") class="sortable">
                            "Amount (Dataset 2)"
                        </th>
                        <th on:click=toggle_sort("diff") class="sortable">
                            "Difference"
                        </th>
                    </tr>
                    <tr class="filter-row">
                        <th>
                            <ColumnFilter
                                placeholder="Filter ID...".to_string()
                                on_change=Callback::new(move |val| set_filter_id.set(val))
                            />
                        </th>
                        <th>
                            <ColumnFilter
                                placeholder="Filter Name...".to_string()
                                on_change=Callback::new(move |val| set_filter_name1.set(val))
                            />
                        </th>
                        <th>
                            <ColumnFilter
                                placeholder="Filter Amount...".to_string()
                                on_change=Callback::new(move |val| set_filter_amount1.set(val))
                            />
                        </th>
                        <th>
                            <ColumnFilter
                                placeholder="Filter Name...".to_string()
                                on_change=Callback::new(move |val| set_filter_name2.set(val))
                            />
                        </th>
                        <th>
                            <ColumnFilter
                                placeholder="Filter Amount...".to_string()
                                on_change=Callback::new(move |val| set_filter_amount2.set(val))
                            />
                        </th>
                        <th>
                            <ColumnFilter
                                placeholder="Filter Diff...".to_string()
                                on_change=Callback::new(move |val| set_filter_diff.set(val))
                            />
                        </th>
                    </tr>
                </thead>
                <tbody>
                    {move || {
                        filtered()
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
                                        <DiffCell value=diff />
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
