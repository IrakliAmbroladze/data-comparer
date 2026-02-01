use data_comparer_shared::Dataset;
use leptos::prelude::*;

#[component]
pub fn FileUpload(on_dataset_loaded: Callback<Dataset>, dataset_name: String) -> impl IntoView {
    view! {
        <div class="file-upload">
            <h3>{dataset_name}</h3>
        </div>
    }
}
