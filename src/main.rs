mod home;

use leptos::*;

fn main() {
    mount_to_body(App)
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        "Salve mundi!"
    }
}
