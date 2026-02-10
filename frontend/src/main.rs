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
    let grid_data1 = RwSignal::new(Vec::<Record>::new());
    let grid_data2 = RwSignal::new(Vec::<Record>::new());

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

    let clear = move |_| {
        grid_data1.set(vec![]);
        grid_data2.set(vec![]);
        set_result.set(None);
    };

    view! {
        <div class="container">
            <header>
                <h1>"Data Comparer"</h1>
                <p>"Compare two datasets by ID"</p>
                <button
                    class="compare-btn"
                    on:click=clear
                    disabled=move || {
                        grid_data1.get().is_empty() && grid_data2.get().is_empty() || loading.get()
                    }
                >
                    {move || if loading.get() { "refreshing..." } else { "clear datasets" }}
                </button>

            </header>

            <main>
                <div class="upload-section">
                    <div>
                        <FileUpload
                            on_dataset_loaded=Callback::new(move |ds: Dataset| {
                                grid_data1.update(|rows| rows.extend(ds.records));
                            })
                            dataset_name="Dataset 1 (Sales)".to_string()
                        />
                        <EditableGrid dataset_name="Dataset 1".to_string() data=grid_data1 />
                    </div>

                    <div>
                        <FileUpload
                            on_dataset_loaded=Callback::new(move |ds: Dataset| {
                                grid_data2.update(|rows| rows.extend(ds.records));
                            })
                            dataset_name="Dataset 2 (Payments)".to_string()
                        />
                        <EditableGrid dataset_name="Dataset 2".to_string() data=grid_data2 />
                    </div>
                </div>
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
