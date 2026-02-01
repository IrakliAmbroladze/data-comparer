use std::{cell::RefCell, rc::Rc};

use data_comparer_shared::Dataset;
use leptos::{ev::Event, logging::log, prelude::*};

#[component]
pub fn FileUpload(on_dataset_loaded: Callback<Dataset>, dataset_name: String) -> impl IntoView {
    let file_ref = Rc::new(RefCell::new(None::<web_sys::File>));
    let (has_file, set_has_file) = signal(false);
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);

    let on_file_change = move |ev: Event| {
        log!("file changed");
    };

    let upload_file = move |_| set_loading.update(|prev| *prev = !*prev);

    view! {
        <div class="file-upload">
            <h3>{dataset_name}</h3>
            <input
                type="file"
                accept=".csv,.xlsx,.xlsm"
                on:change=on_file_change
            />
            <button
                on:click=upload_file
                 disabled=move || !has_file.get() || loading.get()
            >
                {move || if loading.get() {"Uploading..."} else { "Upload" }}
            </button>

            {move || error.get().map(|e| view! {
                <p class="error">{e}</p>
            })}
        </div>
    }
}
