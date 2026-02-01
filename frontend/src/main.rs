use data_comparer_shared::{ComparisonResult, Dataset};
use gloo_net::http::Request;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos::serde_json;

mod components;
use components::file_upload::FileUpload;
use components::results_display::ResultsDisplay;

#[component]
fn App() -> impl IntoView {
    let (dataset1, set_dataset1) = signal(None::<Dataset>);
    let (dataset2, set_dataset2) = signal(None::<Dataset>);
    let (result, set_result) = signal(None::<ComparisonResult>);
    let (loading, set_loading) = signal(false);

    let compare = move |_| {
        if let (Some(ds1), Some(ds2)) = (dataset1.get(), dataset2.get()) {
            set_loading.set(true);

            spawn_local(async move {
                let body = serde_json::json!({
                    "dataset1": ds1,
                    "dataset2": ds2,
                });

                let response = Request::post("http://localhost:3000/compare")
                    .header("Content-Type", "application/json")
                    .body(body.to_string())
                    .expect("Failed to build request")
                    .send()
                    .await;

                match response {
                    Ok(resp) => {
                        if resp.ok() {
                            if let Ok(json) = resp.json::<serde_json::Value>().await {
                                if let Ok(comp_result) = serde_json::from_value::<ComparisonResult>(
                                    json["result"].clone(),
                                ) {
                                    set_result.set(Some(comp_result));
                                }
                            }
                        }
                    }
                    Err(_) => {}
                }

                set_loading.set(false);
            });
        }
    };

    view! {
        <div class="container">
            <header>
                <h1>"Data Comparer"</h1>
                <p>"Compare two datasets by ID"</p>
            </header>

            <main>
                <div class="upload-section">
                    <FileUpload
                        on_dataset_loaded=Callback::new(move |ds| set_dataset1.set(Some(ds)))
                        dataset_name="Dataset 1 (Sales)".to_string()
                    />

                    <FileUpload
                        on_dataset_loaded=Callback::new(move |ds| set_dataset2.set(Some(ds)))
                        dataset_name="Dataset 2 (Payments)".to_string()
                    />
                </div>

                <div class="status">
                    {move || {
                        dataset1
                            .get()
                            .map(|ds| view! { <p>"Dataset 1: " {ds.records.len()} " records"</p> })
                    }}
                    {move || {
                        dataset2
                            .get()
                            .map(|ds| view! { <p>"Dataset 2: " {ds.records.len()} " records"</p> })
                    }}
                </div>

                <button
                    class="compare-btn"
                    on:click=compare
                    disabled=move || {
                        dataset1.get().is_none() || dataset2.get().is_none() || loading.get()
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
