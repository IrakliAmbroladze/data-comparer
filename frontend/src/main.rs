use data_comparer_shared::{ComparisonResult, Dataset, Record};
use gloo_net::http::Request;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos::serde_json;

mod components;
use components::editable_grid::EditableGrid;
use components::file_upload::FileUpload;
use components::results_display::ResultsDisplay;

fn api_url() -> String {
    option_env!("TRUNK_PUBLIC_API_URL")
        .unwrap_or("http://localhost:3000")
        .to_string()
}

#[component]
fn App() -> impl IntoView {
    let (uploaded_data1, set_uploaded_data1) = signal(Vec::<Record>::new());
    let (uploaded_data2, set_uploaded_data2) = signal(Vec::<Record>::new());

    let (grid_data1, set_grid_data1) = signal(Vec::<Record>::new());
    let (grid_data2, set_grid_data2) = signal(Vec::<Record>::new());

    let (result, set_result) = signal(None::<ComparisonResult>);
    let (loading, set_loading) = signal(false);

    let compare = move |_| {
        let ds1 = grid_data1.get();
        let ds2 = grid_data2.get();
        if ds1.is_empty() || ds2.is_empty() {
            return;
        }
        set_loading.set(true);

        spawn_local(async move {
            let body = serde_json::json!({
                "dataset1": {
                    "name": "Dataset 1",
                    "records": ds1
                },
                "dataset2": {
                    "name": "Dataset 2",
                    "records": ds2
                }
            });
            let response = Request::post(&format!("{}/compare", api_url()))
                .header("Content-Type", "application/json")
                .body(body.to_string())
                .expect("Failed to build request")
                .send()
                .await;

            match response {
                Ok(resp) => {
                    if resp.ok() {
                        if let Ok(json) = resp.json::<serde_json::Value>().await {
                            if let Ok(comp_result) =
                                serde_json::from_value::<ComparisonResult>(json["result"].clone())
                            {
                                set_result.set(Some(comp_result));
                            }
                        }
                    }
                }
                Err(_) => {}
            }

            set_loading.set(false);
        });
    };

    view! {
        <div class="container">
            <header>
                <h1>"Data Comparer"</h1>
                <p>"Compare two datasets by ID"</p>
            </header>

            <main>
                <div class="upload-section">
                    <div>
                        <FileUpload
                            on_dataset_loaded=Callback::new(move |ds: Dataset| {
                                set_uploaded_data1.set(ds.records);
                            })
                            dataset_name="Dataset 1 (Sales)".to_string()
                        />

                        <EditableGrid
                            dataset_name="Dataset 1".to_string()
                            initial_data=uploaded_data1.into()
                            on_data_change=Callback::new(move |data| set_grid_data1.set(data))
                        />
                    </div>

                    <div>
                        <FileUpload
                            on_dataset_loaded=Callback::new(move |ds: Dataset| {
                                set_uploaded_data2.set(ds.records);
                            })
                            dataset_name="Dataset 2 (Payments)".to_string()
                        />
                        <EditableGrid
                            dataset_name="Dataset 2".to_string()
                            initial_data=uploaded_data2.into()
                            on_data_change=Callback::new(move |data| set_grid_data2.set(data))
                        />
                    </div>
                </div>

                // <div class="status">
                // {move || {
                // dataset1
                // .get()
                // .map(|ds| view! { <p>"Dataset 1: " {ds.records.len()} " records"</p> })
                // }}
                // {move || {
                // dataset2
                // .get()
                // .map(|ds| view! { <p>"Dataset 2: " {ds.records.len()} " records"</p> })
                // }}
                // </div>

                <button
                    class="compare-btn"
                    on:click=compare
                    disabled=move || {
                        grid_data1.get().is_empty() || grid_data2.get().is_empty() || loading.get()
                    }
                >
                    {move || if loading.get() { "Comparing..." } else { "Compare Datasets" }}
                </button>

                {move || result.get().map(|r| view! { <ResultsDisplay result=r /> })}
            </main>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App /> });
}
