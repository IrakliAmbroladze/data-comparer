use leptos::prelude::*;

#[component]
pub fn FileUpload(dataset_name: String) -> impl IntoView {
    view! {
        <div class="file-upload">
            <h3>{dataset_name}</h3>
        </div>
    }
}
