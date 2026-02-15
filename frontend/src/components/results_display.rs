use crate::components::matched_table::MatchedTable;
use crate::components::unmatched_table::UnmatchedTable;
use data_comparer_shared::ComparisonResult;
use leptos::prelude::*;

#[component]
pub fn ResultsDisplay(result: ComparisonResult) -> impl IntoView {
    view! {
        <div class="results">
            <h2>"Comparison Results"</h2>

            <MatchedTable data=result.matched.clone() />

            <UnmatchedTable
                title=format!("Unmatched from Dataset 1 ({})", result.unmatched_from_first.len())
                data=result.unmatched_from_first.clone()
            />

            <UnmatchedTable
                title=format!("Unmatched from Dataset 2 ({})", result.unmatched_from_second.len())
                data=result.unmatched_from_second.clone()
            />
        </div>
    }
}
