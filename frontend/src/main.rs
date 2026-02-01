use leptos::prelude::*;

mod components;
use components::file_upload::FileUpload;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="container">
            <header>
                <h1>"Data Comparer"</h1>
                <p>"Compare two datasets by ID"</p>
            </header>

            <main>
                <p>"Frontend is ready"</p>
                    <FileUpload dataset_name="Dataset 1 (Sales)".to_string()/>
                    <FileUpload dataset_name="Dataset 2 (Payments)".to_string()/>
                <div class="upload-section">
                </div>
            </main>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App/> });
}
