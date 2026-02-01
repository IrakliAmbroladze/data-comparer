use data_comparer_shared::Dataset;
use leptos::prelude::*;

mod components;
use components::file_upload::FileUpload;

#[component]
fn App() -> impl IntoView {
    let (dataset1, set_dataset1) = signal(None::<Dataset>);
    let (dataset2, set_dataset2) = signal(None::<Dataset>);
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
                {move || dataset1.get().map(|ds| view! {
                    <p>"Dataset 1: " {ds.records.len()} " records"</p>
                })}
                {move || dataset2.get().map(|ds| view! {
                    <p>"Dataset 2: " {ds.records.len()} " records"</p>
                })}

            </main>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App/> });
}
