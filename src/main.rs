mod home;
mod nav;

use leptos::*;
use leptos_router::*;

fn main() {
    mount_to_body(App)
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router trailing_slash=leptos_router::TrailingSlash::Redirect>
            <main class="bg-darker text-txt">
                <nav::Navigation></nav::Navigation>
                <Routes>
                    <Route path="/" view=home::Home/>
                </Routes>
            </main>
        </Router>
    }
}
