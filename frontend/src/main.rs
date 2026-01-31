use leptos::prelude::*;

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
            </main>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App/> });
}
