use leptos::*;

use crate::account::*;

#[component]
pub fn Home() -> impl IntoView {
    let user = expect_context::<User>();

    view! {
        <div class="w-full px-auto py-4 bg-header text-center">
            <h1 class="text-xl">"Salve Mundi!"</h1>
            <p class="text-lg mt-6">
                {move || if user.logged_in() {
                    view! {
                        "Hallo '" {user.username} "' met wachtwoord '" {user.password} "'"
                    }.into_view()
                } else {
                    view! {
                        "Hallo gast"
                    }.into_view()
                }}
            </p>
        </div>
    }
}
