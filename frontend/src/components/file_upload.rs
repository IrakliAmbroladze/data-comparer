use data_comparer_shared::Dataset;
use leptos::{ev::Event, logging::log, prelude::*};

#[component]
pub fn FileUpload(on_dataset_loaded: Callback<Dataset>, dataset_name: String) -> impl IntoView {
    let on_file_change = move |ev: Event| {
        log!("file changed");
    };
    view! {
        <div class="file-upload">
            <h3>{dataset_name}</h3>
            <input
                type="file"
                accept=".csv,.xlsx,.xlsm"
                on:change=on_file_change
            />
        </div>
    }
}
