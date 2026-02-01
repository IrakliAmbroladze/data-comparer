use data_comparer_shared::Dataset;
use gloo_net::http::Request;
use leptos::{ev::Event, prelude::*, reactive::spawn_local, serde_json};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::FormData;

#[component]
pub fn FileUpload(on_dataset_loaded: Callback<Dataset>, dataset_name: String) -> impl IntoView {
    let file_ref = Rc::new(RefCell::new(None::<web_sys::File>));
    let (has_file, set_has_file) = signal(false);
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);

    let on_file_change = {
        let file_ref = file_ref.clone();
        move |ev: Event| {
            let input = ev
                .target()
                .unwrap()
                .dyn_into::<web_sys::HtmlInputElement>()
                .unwrap();
            if let Some(files) = input.files() {
                if let Some(f) = files.get(0) {
                    *file_ref.borrow_mut() = Some(f);
                    set_has_file.set(true);
                    set_error.set(None);
                }
            }
        }
    };

    let upload_file = {
        let file_ref = file_ref.clone();
        move |_| {
            if let Some(file) = file_ref.borrow().as_ref() {
                let file = file.clone();
                set_loading.set(true);
                set_error.set(None);

                spawn_local(async move {
                    let form_data = FormData::new().unwrap();
                    form_data.append_with_blob("file", &file).unwrap();

                    let response = Request::post("http://localhost:3000/upload")
                        .body(form_data)
                        .expect("Failed to build request")
                        .send()
                        .await;

                    match response {
                        Ok(resp) => {
                            if resp.ok() {
                                match resp.json::<serde_json::Value>().await {
                                    Ok(json) => {
                                        if let Ok(dataset) = serde_json::from_value::<Dataset>(
                                            json["dataset"].clone(),
                                        ) {
                                            on_dataset_loaded.run(dataset);
                                        }
                                    }
                                    Err(e) => set_error.set(Some(format!("Parse error: {}", e))),
                                }
                            } else {
                                set_error.set(Some("Upload failed".to_string()));
                            }
                        }
                        Err(e) => set_error.set(Some(format!("Network error: {}", e))),
                    }

                    set_loading.set(false);
                });
            }
        }
    };

    view! {
        <div class="file-upload">
            <h3>{dataset_name}</h3>
            <input type="file" accept=".csv,.xlsx,.xlsm" on:change=on_file_change />
            <button on:click=upload_file disabled=move || !has_file.get() || loading.get()>
                {move || if loading.get() { "Uploading..." } else { "Upload" }}
            </button>

            {move || error.get().map(|e| view! { <p class="error">{e}</p> })}
        </div>
    }
}
